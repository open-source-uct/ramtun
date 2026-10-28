mod answers;
mod grading;
mod questions;
pub mod warnings;

use crate::{
	attempts::*,
	banks::QuestionId,
	courses::CoursePolicy,
	quizzes::{QuizError, QuizId, QuizKind, QuizRepository},
	shared::AppResult,
	users::{User, UserId, UserRepository, UsersError},
};

use chrono::{Duration, Utc};
use futures::TryFutureExt;
use grading::GradingOutput;
use std::collections::HashMap;
use std::sync::Arc;
use sword::prelude::*;

pub use answers::AnswerService;
pub use grading::GradingService;
pub use questions::QuestionService;

#[injectable]
pub struct AttemptsService {
	repository: Arc<AttemptRepository>,
	quizzes: Arc<QuizRepository>,
	course_policy: Arc<CoursePolicy>,
	questions: Arc<QuestionService>,
	grading: Arc<GradingService>,
	users: Arc<UserRepository>,
	answers: Arc<AnswerService>,
}

impl AttemptsService {
	pub async fn list_attempts(
		&self,
		current_user: &User,
		filter: AttemptFilter,
	) -> AppResult<Vec<AttemptListItemView>> {
		self.course_policy
			.require_manager_member(current_user, &filter.course_id)
			.await?;

		self.repository.list_attempts_managed(filter).await
	}

	/// Returns the corresponding `Attempt` by its given id.
	/// Also checks if the user is the owner of the attempt returning error if it's not.
	pub async fn get_attempt_for_user(
		&self,
		attempt_id: AttemptId,
		user_id: UserId,
	) -> AppResult<Attempt> {
		let Some(attempt) = self.repository.find_by_id(&attempt_id).await? else {
			return Err(AttemptError::NotFound(attempt_id))?;
		};

		if attempt.student_id != user_id {
			Err(AttemptError::Forbidden)?;
		}

		Ok(attempt)
	}

	/// Initializes a new attempt for a given quiz and user.
	/// If the user already has an active (in-progress) attempt, it returns that one.
	/// If the user already submitted or completed this quiz, returns an error.
	pub async fn initialize_attempt(
		&self,
		quiz_id: QuizId,
		user_id: UserId,
	) -> AppResult<AttemptView> {
		if let Some(active) = self.repository.find_active_by_user(&user_id).await? {
			let quiz = self
				.quizzes
				.find_by_id(&active.quiz_id)
				.await?
				.ok_or(QuizError::NotFound(active.quiz_id))?;

			let questions = self
				.questions
				.get_quiz_questions(&active.quiz_id)
				.and_then(|q| self.questions.filter_questions(q, &active.question_order))
				.and_then(|q| self.questions.into_views(q))
				.await?;

			let answers = self.get_saved_answer_views(&active.id).await?;

			return Ok(AttemptView::from((
				active, questions, quiz.kind, quiz.title, answers,
			)));
		}

		let Some(quiz) = self.quizzes.find_by_id(&quiz_id).await? else {
			return Err(QuizError::NotFound(quiz_id))?;
		};

		let now = Utc::now();

		if quiz.starts_at > now {
			Err(AttemptError::QuizNotStarted)?;
		}

		if quiz.results_published_at.is_some() {
			Err(AttemptError::QuizEnded)?;
		}

		let filter = AttemptFilter {
			course_id: quiz.course_id,
			quiz_id: quiz.id,
			student_id: Some(user_id),
		};

		let attempts = self.repository.list_attempts(filter).await?;

		if !attempts.is_empty() {
			Err(AttemptError::AlreadyAttempted)?;
		}

		let question_order = self
			.questions
			.get_question_ids_for_attempt(&quiz.id, quiz.question_count as usize)
			.await?;

		let expires_at = now + Duration::minutes(quiz.attempt_duration_minutes.into());

		let attempt = Attempt::builder()
			.student_id(user_id)
			.quiz_id(quiz.id)
			.question_order(question_order.clone())
			.started_at(now)
			.expires_at(expires_at)
			.build();

		self.repository.save(&attempt).await?;

		let questions = self
			.questions
			.get_quiz_questions(&quiz.id)
			.and_then(|q| self.questions.filter_questions(q, &question_order))
			.and_then(|q| self.questions.into_views(q))
			.await?;

		Ok(AttemptView::from((
			attempt,
			questions,
			quiz.kind,
			quiz.title,
			Vec::new(),
		)))
	}

	pub async fn get_active_attempt(&self, user_id: &UserId) -> AppResult<Option<AttemptView>> {
		let Some(active) = self.repository.find_active_by_user(user_id).await? else {
			return Ok(None);
		};

		let quiz = self
			.quizzes
			.find_by_id(&active.quiz_id)
			.await?
			.ok_or(QuizError::NotFound(active.quiz_id))?;

		let questions = self
			.questions
			.get_quiz_questions(&active.quiz_id)
			.and_then(|q| self.questions.filter_questions(q, &active.question_order))
			.and_then(|q| self.questions.into_views(q))
			.await?;

		let answers = self.get_saved_answer_views(&active.id).await?;

		Ok(Some(AttemptView::from((
			active, questions, quiz.kind, quiz.title, answers,
		))))
	}

	pub async fn get_attempt_view(
		&self,
		attempt_id: AttemptId,
		user_id: UserId,
	) -> AppResult<AttemptView> {
		let attempt = self.get_attempt_for_user(attempt_id, user_id).await?;

		let quiz = self
			.quizzes
			.find_by_id(&attempt.quiz_id)
			.await?
			.ok_or(QuizError::NotFound(attempt.quiz_id))?;

		let questions = self
			.questions
			.get_quiz_questions(&attempt.quiz_id)
			.and_then(|q| self.questions.filter_questions(q, &attempt.question_order))
			.and_then(|q| self.questions.into_views(q))
			.await?;

		let answers = self.get_saved_answer_views(&attempt.id).await?;

		Ok(AttemptView::from((
			attempt, questions, quiz.kind, quiz.title, answers,
		)))
	}

	async fn get_saved_answer_views(
		&self,
		attempt_id: &AttemptId,
	) -> AppResult<Vec<SavedAnswerView>> {
		let answers = self.repository.list_attempt_answers(attempt_id).await?;
		Ok(answers.into_iter().map(SavedAnswerView::from).collect())
	}

	pub async fn save_answer(&self, user_id: UserId, input: SaveAttemptAnswerDto) -> AppResult<()> {
		let attempt = self
			.get_attempt_for_user(input.attempt_id, user_id)
			.and_then(|attempt| self.ensure_attempt_is_answerable(attempt))
			.await?;

		let Some(quiz) = self.quizzes.find_by_id(&attempt.quiz_id).await? else {
			return Err(QuizError::NotFound(attempt.quiz_id))?;
		};

		match quiz.kind {
			QuizKind::Certainty if input.certainty_level.is_none() => {
				Err(AttemptError::CertaintyLevelRequired)?;
			}
			QuizKind::Traditional if input.certainty_level.is_some() => {
				Err(AttemptError::CertaintyLevelNotAllowed)?;
			}
			_ => {}
		}

		self.questions
			.ensure_question_belongs_to_attempt(&attempt.question_order, &input.question_id)?;

		let question = self
			.questions
			.get_question_by_id(&attempt.quiz_id, &input.question_id)
			.await?
			.ok_or(QuizError::QuestionNotFound(input.question_id))?;

		self.questions
			.ensure_valid_answer_index(&question, input.answer_index)?;

		let answer = AttemptAnswer {
			attempt_id: attempt.id,
			question_id: input.question_id,
			answer_index: input.answer_index,
			certainty_level: input.certainty_level,
			is_correct: None,
			awarded_points: None,
		};

		self.repository.upsert_attempt_answer(&answer).await?;

		Ok(())
	}

	pub async fn submit_attempt(
		&self,
		attempt_id: AttemptId,
		user_id: UserId,
	) -> AppResult<AttemptSubmitView> {
		let mut attempt = self
			.get_attempt_for_user(attempt_id, user_id)
			.and_then(|attempt| self.ensure_attempt_is_answerable(attempt))
			.await?;

		let Some(quiz) = self.quizzes.find_by_id(&attempt.quiz_id).await? else {
			return Err(QuizError::NotFound(attempt.quiz_id))?;
		};

		let attempt_q_order = &attempt.question_order;

		let attempt_questions = self
			.questions
			.get_quiz_questions(&attempt.quiz_id)
			.and_then(|q| self.questions.filter_questions(q, attempt_q_order))
			.and_then(|q| self.questions.into_map(q))
			.await?;

		let submitted_answers = self
			.answers
			.get_attempt_answers(&attempt_id)
			.and_then(|ans| self.answers.into_map(ans))
			.await?;

		// Grade the attempt based on the attempt questions and submitted answers
		// p: the obtained attempts points with a max of quiz.max_score.
		// g: the obtained grade based on "((p / p_max) * 6) + 1" CL scale.

		let grading = self
			.grading
			.grade_attempt(&quiz, &attempt_questions, &submitted_answers)
			.await?;

		self.persist_attempt_grading(&mut attempt, &grading, &submitted_answers)
			.await?;

		Ok(AttemptSubmitView {
			attempt_id: attempt.id,
			quiz_id: attempt.quiz_id,
			submitted_at: attempt.submitted_at.ok_or(AttemptError::AlreadySubmitted)?,
		})
	}

	pub async fn view_results(
		&self,
		attempt_id: AttemptId,
		user: &User,
	) -> AppResult<AttemptResultView> {
		let mut attempt = self.get_attempt_for_user(attempt_id, user.id).await?;

		let submitted_at = attempt
			.submitted_at
			.ok_or(AttemptError::ResultsNotAvailable)?;

		let Some(quiz) = self.quizzes.find_by_id(&attempt.quiz_id).await? else {
			return Err(QuizError::NotFound(attempt.quiz_id))?;
		};

		if quiz.results_published_at.is_none() {
			Err(QuizError::ResultsNotPublished)?;
		}

		let score_points = attempt.score.ok_or(AttemptError::ResultsNotAvailable)?;
		let grade = attempt.grade.ok_or(AttemptError::ResultsNotAvailable)?;

		let question_results = self.build_question_result_views(&attempt).await?;

		self.repository.mark_results_viewed(&attempt.id).await?;

		attempt.results_viewed_at = Some(Utc::now());

		Ok(AttemptResultView {
			attempt_id: attempt.id,
			student_id: attempt.student_id,
			user_name: user.name.clone(),
			quiz_id: attempt.quiz_id,
			submitted_at,
			score: score_points,
			max_score: quiz.max_score,
			grade,
			results_viewed_at: attempt.results_viewed_at,
			questions: question_results,
		})
	}

	pub async fn view_results_by_join_code(
		&self,
		join_code: &str,
		user: &User,
	) -> AppResult<AttemptResultView> {
		let Some(quiz) = self.quizzes.find_by_code(join_code).await? else {
			return Err(QuizError::InvalidCode)?;
		};

		if quiz.results_published_at.is_none() {
			Err(QuizError::ResultsNotPublished)?;
		}

		let attempt = self
			.repository
			.find_by_quiz_and_student(&quiz.id, &user.id)
			.await?
			.ok_or(AttemptError::NoActiveAttempts)?;

		if attempt.results_viewed_at.is_some() && !quiz.results_multiple_views {
			Err(AttemptError::ResultsAlreadyViewed)?;
		}

		self.view_results(attempt.id, user).await
	}

	pub async fn view_attempt_results_managed(
		&self,
		current_user: &User,
		attempt_id: AttemptId,
	) -> AppResult<AttemptResultView> {
		let attempt = self
			.repository
			.find_by_id(&attempt_id)
			.await?
			.ok_or(AttemptError::NotFound(attempt_id))?;

		let quiz = self
			.quizzes
			.find_by_id(&attempt.quiz_id)
			.await?
			.ok_or(QuizError::NotFound(attempt.quiz_id))?;

		self.course_policy
			.require_accessible_course(current_user, &quiz.course_id)
			.await?;

		let submitted_at = attempt.submitted_at.ok_or(AttemptError::NotSubmitted)?;

		let score_points = attempt.score.ok_or(AttemptError::ResultsNotAvailable)?;
		let grade = attempt.grade.ok_or(AttemptError::ResultsNotAvailable)?;

		let question_results = self.build_question_result_views(&attempt).await?;

		let student = self
			.users
			.find_by_id(&attempt.student_id)
			.await?
			.ok_or(UsersError::NotFound(attempt.student_id))?;

		Ok(AttemptResultView {
			attempt_id: attempt.id,
			student_id: attempt.student_id,
			user_name: student.name.clone(),
			quiz_id: attempt.quiz_id,
			submitted_at,
			score: score_points,
			max_score: quiz.max_score,
			grade,
			results_viewed_at: attempt.results_viewed_at,
			questions: question_results,
		})
	}

	async fn persist_attempt_grading(
		&self,
		attempt: &mut Attempt,
		grading: &GradingOutput,
		submitted_answers: &HashMap<QuestionId, AttemptAnswer>,
	) -> AppResult<()> {
		attempt.grade = Some(grading.grade);
		attempt.score = Some(grading.score_points);
		attempt.submitted_at = Some(Utc::now());

		for result in &grading.question_results {
			if let Some(answer) = submitted_answers.get(&result.question_id) {
				let mut updated_answer = answer.clone();

				updated_answer.is_correct = Some(result.is_correct);
				updated_answer.awarded_points = Some(result.awarded_points);

				self.repository
					.upsert_attempt_answer(&updated_answer)
					.await?;
			}
		}

		self.repository.save(attempt).await?;

		Ok(())
	}

	async fn build_question_result_views(
		&self,
		attempt: &Attempt,
	) -> AppResult<Vec<QuestionResultView>> {
		let questions_by_id = self
			.questions
			.get_quiz_questions(&attempt.quiz_id)
			.and_then(|q| self.questions.filter_questions(q, &attempt.question_order))
			.and_then(|qs| self.questions.into_map(qs))
			.await?;

		let answers = self.repository.list_attempt_answers(&attempt.id).await?;

		let answers_by_id: HashMap<QuestionId, AttemptAnswer> = answers
			.into_iter()
			.map(|answer| (answer.question_id, answer))
			.collect();

		let mut question_results = Vec::new();

		for question_id in &attempt.question_order {
			let question = questions_by_id
				.get(question_id)
				.ok_or(QuizError::QuestionNotFound(*question_id))?;
			let answer = answers_by_id.get(question_id);

			let (answer_index, certainty_level, is_correct, awarded_points) = match answer {
				Some(answer) => (
					Some(answer.answer_index),
					answer.certainty_level,
					answer.is_correct.ok_or(AttemptError::ResultsNotAvailable)?,
					answer
						.awarded_points
						.ok_or(AttemptError::ResultsNotAvailable)?,
				),
				None => (None, None, false, 0),
			};

			question_results.push(QuestionResultView {
				question_id: *question_id,
				question: question.prompt.clone(),
				options: question.options.clone(),
				answer_index,
				correct_answer_index: question.answer_index,
				justification: question.justification.clone(),
				certainty_level,
				is_correct,
				awarded_points,
			});
		}

		Ok(question_results)
	}

	/// Ensures that the given attempt is in a state where it can be answered.
	/// It checks if the attempt is not expired and not already submitted.
	async fn ensure_attempt_is_answerable(&self, attempt: Attempt) -> AppResult<Attempt> {
		let now = Utc::now();

		if attempt.expires_at + Duration::minutes(1) < now {
			Err(AttemptError::Expired)?;
		}

		if attempt.submitted_at.is_some() {
			Err(AttemptError::AlreadySubmitted)?;
		}

		Ok(attempt)
	}
}
