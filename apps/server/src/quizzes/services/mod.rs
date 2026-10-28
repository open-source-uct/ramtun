mod codegen;
mod policy;

pub use codegen::*;
pub use policy::*;

use std::sync::Arc;

use crate::attempts::AttemptRepository;
use crate::banks::{QuestionBankRepository, QuestionRepository};
use crate::courses::{CourseId, CourseRepository, CoursesError};
use crate::quizzes::*;
use crate::shared::{AppResult, TransactionManager};
use crate::users::{User, UserRole};

use chrono::{DateTime, Utc};
use sword::prelude::*;

#[injectable]
pub struct QuizService {
	codegen: Arc<QuizCodeGenerator>,
	policy: Arc<QuizPolicy>,
	repository: Arc<QuizRepository>,
	courses: Arc<CourseRepository>,
	tx: Arc<TransactionManager>,
	banks: Arc<QuestionBankRepository>,
	questions: Arc<QuestionRepository>,
	attempts: Arc<AttemptRepository>,
}

impl QuizService {
	pub async fn get_one(&self, current_user: &User, quiz_id: &QuizId) -> AppResult<QuizView> {
		let quiz = self
			.policy
			.require_managed_quiz(current_user, quiz_id)
			.await?;

		let course = self
			.courses
			.find_by_id(&quiz.course_id)
			.await?
			.ok_or_else(|| CoursesError::NotFound(quiz.course_id.to_string()))?;

		Ok(QuizView::from((quiz, course)))
	}

	pub async fn list_by_course(
		&self,
		current_user: &User,
		course_id: &CourseId,
	) -> AppResult<Vec<QuizView>> {
		if current_user.role != UserRole::Admin
			&& !self.courses.is_member(course_id, &current_user.id).await?
		{
			Err(QuizError::Forbidden)?;
		}

		let quizzes = self.repository.list_by_course(course_id).await?;

		let course = self
			.courses
			.find_by_id(course_id)
			.await?
			.ok_or_else(|| CoursesError::NotFound(course_id.to_string()))?;

		let views = quizzes
			.into_iter()
			.map(|quiz| QuizView::from((quiz, course.clone())))
			.collect::<Vec<_>>();

		Ok(views)
	}

	pub async fn get_join_preview(&self, code: &str) -> AppResult<JoinQuizPreviewView> {
		let Some(quiz) = self.repository.find_by_code(code).await? else {
			return Err(QuizError::InvalidCode)?;
		};

		if quiz.results_published_at.is_some() {
			Err(QuizError::Closed)?;
		}

		Ok(JoinQuizPreviewView::from(&quiz))
	}

	pub async fn close_and_publish_results(
		&self,
		current_user: &User,
		quiz_id: &QuizId,
	) -> AppResult<()> {
		let quiz = self
			.policy
			.require_managed_quiz(current_user, quiz_id)
			.await?;

		if quiz.results_published_at.is_some() {
			Err(QuizError::Closed)?;
		}

		if !self.repository.publish_results(&quiz.id).await? {
			Err(QuizError::Closed)?;
		}

		Ok(())
	}

	pub async fn create(&self, current_user: &User, input: CreateQuizDto) -> AppResult<QuizView> {
		self.policy
			.check_can_create_quiz(current_user, &input.course_id)
			.await?;

		if !self
			.banks
			.are_banks_in_course(&input.bank_ids, &input.course_id)
			.await?
		{
			Err(QuizError::InvalidBanksForCourse)?;
		}

		let questions = self.questions.list_by_bank_ids(&input.bank_ids).await?;

		if input.question_count as usize > questions.len() {
			Err(QuizError::InvalidQuestionCount)?;
		}

		let starts_at = DateTime::parse_from_rfc3339(&input.starts_at)
			.map_err(|_| QuizError::InvalidStartTime)?
			.with_timezone(&Utc);

		let certainty_table = input.certainty_config.map(CertaintyTable::from);
		let max_score =
			Quiz::compute_max_score(input.kind, input.question_count, certainty_table.as_ref());

		let quiz = Quiz::builder()
			.course_id(input.course_id)
			.title(input.title)
			.kind(input.kind)
			.join_code(self.codegen.generate_unique_join_code().await?)
			.question_count(input.question_count)
			.maybe_certainty_table(certainty_table)
			.attempt_duration_minutes(input.attempt_duration_minutes)
			.starts_at(starts_at)
			.results_multiple_views(input.results_multiple_views)
			.created_at(Utc::now())
			.max_score(max_score)
			.build();

		let mut tx = self.tx.begin().await?;

		let quiz = self.repository.save(&mut tx, &quiz).await?;

		let question_ids = questions.into_iter().map(|q| q.id).collect::<Vec<_>>();
		self.repository
			.set_quiz_questions(&mut tx, &quiz.id, &question_ids)
			.await?;

		self.repository
			.set_bank_links(&mut tx, &quiz.id, &input.bank_ids)
			.await?;

		tx.commit().await?;

		let course = self
			.courses
			.find_by_id(&quiz.course_id)
			.await?
			.ok_or_else(|| CoursesError::NotFound(quiz.course_id.to_string()))?;

		Ok(QuizView::from((quiz, course)))
	}

	/// Title and results visibility can change at any time. Anything that
	/// reshapes the evaluation (schedule, duration, size, scoring) is locked
	/// once the quiz has attempts or its results are published.
	pub async fn update(
		&self,
		current_user: &User,
		quiz_id: &QuizId,
		input: UpdateQuizDto,
	) -> AppResult<QuizView> {
		let quiz = self
			.policy
			.require_managed_quiz(current_user, quiz_id)
			.await?;

		let mut updated = quiz.clone();

		if let Some(title) = input.title {
			updated.title = title;
		}

		if let Some(multiple_views) = input.results_multiple_views {
			updated.results_multiple_views = multiple_views;
		}

		if let Some(starts_at) = &input.starts_at {
			updated.starts_at = DateTime::parse_from_rfc3339(starts_at)
				.map_err(|_| QuizError::InvalidStartTime)?
				.with_timezone(&Utc);
		}

		if let Some(duration) = input.attempt_duration_minutes {
			updated.attempt_duration_minutes = duration;
		}

		if let Some(question_count) = input.question_count {
			updated.question_count = question_count;
		}

		if let Some(config) = input.certainty_config {
			if quiz.kind != QuizKind::Certainty {
				Err(QuizError::InvalidQuizMode)?;
			}

			updated.certainty_table = Some(CertaintyTable::from(config));
		}

		let reshapes_quiz = updated.starts_at != quiz.starts_at
			|| updated.attempt_duration_minutes != quiz.attempt_duration_minutes
			|| updated.question_count != quiz.question_count
			|| updated.certainty_table != quiz.certainty_table;

		if reshapes_quiz {
			if quiz.results_published_at.is_some()
				|| self.attempts.exists_for_quiz(&quiz.id).await?
			{
				Err(QuizError::LockedForAttempts)?;
			}

			if updated.starts_at != quiz.starts_at && updated.starts_at < Utc::now() {
				Err(QuizError::InvalidStartTime)?;
			}

			if updated.question_count != quiz.question_count {
				let available = self.questions.list_for_quiz(&quiz.id).await?.len();

				if updated.question_count as usize > available {
					Err(QuizError::InvalidQuestionCount)?;
				}
			}

			updated.max_score = Quiz::compute_max_score(
				updated.kind,
				updated.question_count,
				updated.certainty_table.as_ref(),
			);
		}

		let mut tx = self.tx.begin().await?;
		let saved = self.repository.save(&mut tx, &updated).await?;
		tx.commit().await?;

		let course = self
			.courses
			.find_by_id(&saved.course_id)
			.await?
			.ok_or_else(|| CoursesError::NotFound(saved.course_id.to_string()))?;

		Ok(QuizView::from((saved, course)))
	}

	pub async fn delete_quiz(&self, current_user: &User, quiz_id: &QuizId) -> AppResult<()> {
		let quiz = self
			.policy
			.require_managed_quiz(current_user, quiz_id)
			.await?;

		if !self.repository.delete_by_id(&quiz.id).await? {
			Err(QuizError::NotFound(*quiz_id))?;
		}

		Ok(())
	}
}
