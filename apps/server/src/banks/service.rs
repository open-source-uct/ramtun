use crate::banks::*;
use crate::courses::CourseId;
use crate::quizzes::{Quiz, QuizRepository};
use crate::shared::{AppResult, TransactionManager, Tx};
use crate::users::User;

use chrono::{DateTime, Utc};
use std::sync::Arc;
use sword::prelude::*;

#[injectable]
pub struct QuestionBankService {
	policy: Arc<QuestionBankPolicy>,
	repository: Arc<QuestionBankRepository>,
	questions: Arc<QuestionRepository>,
	quizzes: Arc<QuizRepository>,
	tx: Arc<TransactionManager>,
}

impl QuestionBankService {
	pub async fn list_for_course(
		&self,
		current_user: &User,
		course_id: &CourseId,
	) -> AppResult<Vec<QuestionBankView>> {
		self.policy
			.require_accessible_course(current_user, course_id)
			.await?;

		let banks = self.repository.list_by_course(course_id).await?;

		let mut views = Vec::with_capacity(banks.len());

		for bank in banks {
			let questions = self.questions.list_current_by_bank(&bank.id).await?;

			views.push(QuestionBankView {
				id: bank.id,
				course_id: bank.course_id,
				name: bank.name,
				questions,
				created_at: bank.created_at,
				deleted_at: bank.deleted_at,
			});
		}

		Ok(views)
	}

	pub async fn get_one(
		&self,
		current_user: &User,
		bank_id: &QuestionBankId,
	) -> AppResult<QuestionBankView> {
		let bank = self
			.policy
			.require_accessible_bank(current_user, bank_id)
			.await?;

		let questions = self.questions.list_current_by_bank(&bank.id).await?;

		Ok(QuestionBankView {
			id: bank.id,
			course_id: bank.course_id,
			name: bank.name,
			questions,
			created_at: bank.created_at,
			deleted_at: bank.deleted_at,
		})
	}

	pub async fn create(&self, current_user: &User, input: CreateQuestionBankDto) -> AppResult<()> {
		self.policy
			.require_accessible_course(current_user, &input.course_id)
			.await?;

		let now = Utc::now();

		let bank = QuestionBank::builder()
			.course_id(input.course_id)
			.name(input.name)
			.created_at(now)
			.build();

		let mut tx = self.tx.begin().await?;

		let bank = self.repository.save(&mut tx, &bank).await?;

		let questions = input
			.questions
			.iter()
			.enumerate()
			.map(|(position, q)| build_question(bank.id, position as i16, now, q))
			.collect::<Vec<_>>();

		self.questions
			.insert_generation(&mut tx, &questions)
			.await?;

		tx.commit().await?;

		Ok(())
	}

	pub async fn update(
		&self,
		current_user: &User,
		bank_id: &QuestionBankId,
		input: UpdateQuestionBankDto,
	) -> AppResult<()> {
		let mut bank = self
			.policy
			.require_accessible_bank(current_user, bank_id)
			.await?;

		if let Some(name) = input.name {
			bank.name = name;
		}

		let linked_quizzes = self.questions.list_linked_quizzes(&bank.id).await?;
		self.ensure_not_linked_to_running_quiz(&linked_quizzes)?;

		let mut tx = self.tx.begin().await?;

		self.repository.save(&mut tx, &bank).await?;

		if let Some(question_inputs) = input.questions {
			let current = self.questions.list_current_by_bank(&bank.id).await?;
			self.ensure_editable_payload(&current, &question_inputs)?;

			self.questions
				.archive_or_purge_generation(&mut tx, &bank.id)
				.await?;

			let now = Utc::now();

			let questions = question_inputs
				.iter()
				.enumerate()
				.map(|(position, q)| {
					let answer_index = current
						.get(position)
						.map(|question| question.answer_index)
						.unwrap_or(0);
					let justification = match &q.justification {
						PatchField::Missing => current
							.get(position)
							.and_then(|question| question.justification.clone()),
						PatchField::Clear => None,
						PatchField::Set(value) => normalize_justification(Some(value)),
					};

					Question::builder()
						.bank_id(bank.id)
						.position(position as i16)
						.prompt(q.prompt.clone())
						.options(q.options.clone())
						.answer_index(answer_index)
						.maybe_justification(justification)
						.created_at(now)
						.build()
				})
				.collect::<Vec<_>>();

			self.questions
				.insert_generation(&mut tx, &questions)
				.await?;

			self.repoint_not_started_quizzes(&mut tx, &linked_quizzes)
				.await?;
		}

		tx.commit().await?;

		Ok(())
	}

	pub async fn soft_delete(
		&self,
		current_user: &User,
		bank_id: &QuestionBankId,
	) -> AppResult<()> {
		let bank = self
			.policy
			.require_accessible_bank(current_user, bank_id)
			.await?;

		let linked_quizzes = self.questions.list_linked_quizzes(&bank.id).await?;
		self.ensure_not_linked_to_running_quiz(&linked_quizzes)?;

		let mut tx = self.tx.begin().await?;

		for quiz in &linked_quizzes {
			if !self.is_not_started(quiz) {
				continue;
			}

			let remaining = self
				.questions
				.list_questions_for_linked_banks_excluding(&quiz.id, &bank.id)
				.await?;

			self.ensure_question_count(&remaining, quiz)?;

			let ids = remaining.into_iter().map(|q| q.id).collect::<Vec<_>>();
			self.quizzes
				.set_quiz_questions(&mut tx, &quiz.id, &ids)
				.await?;
		}

		self.questions
			.archive_or_purge_generation(&mut tx, &bank.id)
			.await?;

		if !self.repository.soft_delete(&mut tx, &bank.id).await? {
			Err(QuestionBankError::NotFound(bank.id.to_string()))?;
		}

		tx.commit().await?;

		Ok(())
	}

	/// Recomputes the question selection of every linked quiz that has not
	/// started yet, so it points to the bank's new generation.
	async fn repoint_not_started_quizzes(
		&self,
		tx: &mut Tx<'_>,
		quizzes: &[Quiz],
	) -> AppResult<()> {
		for quiz in quizzes {
			if !self.is_not_started(quiz) {
				continue;
			}

			let current = self
				.questions
				.list_questions_for_linked_banks(&quiz.id)
				.await?;

			self.ensure_question_count(&current, quiz)?;

			let ids = current.into_iter().map(|q| q.id).collect::<Vec<_>>();
			self.quizzes.set_quiz_questions(tx, &quiz.id, &ids).await?;
		}

		Ok(())
	}

	fn is_not_started(&self, quiz: &Quiz) -> bool {
		let now = Utc::now();
		quiz.results_published_at.is_none() && quiz.starts_at > now
	}

	fn ensure_question_count(&self, questions: &[Question], quiz: &Quiz) -> AppResult<()> {
		if questions.len() >= quiz.question_count as usize {
			return Ok(());
		}

		Err(QuestionBankError::InvalidQuestionCountAfterBankUpdate)?
	}

	/// Editing only changes prompt and option texts, so the number of questions
	/// and the number of options of each question must stay unchanged. The
	/// correct answer is preserved server-side by position.
	fn ensure_editable_payload(
		&self,
		current: &[Question],
		incoming: &[UpdateQuestionInput],
	) -> AppResult<()> {
		if current.len() != incoming.len() {
			Err(QuestionBankError::QuestionCountMismatch)?;
		}

		for (index, question) in incoming.iter().enumerate() {
			if current[index].options.len() != question.options.len() {
				Err(QuestionBankError::OptionsCountMismatch)?;
			}
		}

		Ok(())
	}

	fn ensure_not_linked_to_running_quiz(&self, quizzes: &[Quiz]) -> AppResult<()> {
		let now = Utc::now();

		if quizzes
			.iter()
			.any(|quiz| quiz.results_published_at.is_none() && quiz.starts_at <= now)
		{
			Err(QuestionBankError::LockedByRunningQuiz)?;
		}

		Ok(())
	}
}

fn build_question(
	bank_id: QuestionBankId,
	position: i16,
	created_at: DateTime<Utc>,
	input: &QuestionInput,
) -> Question {
	Question::builder()
		.bank_id(bank_id)
		.position(position)
		.prompt(input.prompt.clone())
		.options(input.options.clone())
		.answer_index(input.answer_index as i16)
		.maybe_justification(normalize_justification(input.justification.as_deref()))
		.created_at(created_at)
		.build()
}

/// Trims the justification and stores `None` when nothing is left, so blank
/// explanations are treated as absent.
fn normalize_justification(justification: Option<&str>) -> Option<String> {
	justification
		.map(str::trim)
		.filter(|text| !text.is_empty())
		.map(str::to_owned)
}

#[cfg(test)]
mod tests {
	use super::normalize_justification;

	#[test]
	fn normalize_justification_drops_blank_and_trims_text() {
		assert_eq!(normalize_justification(Some("   ")), None);
		assert_eq!(normalize_justification(None), None);
		assert_eq!(
			normalize_justification(Some("  Porque $x = 2$  ")),
			Some("Porque $x = 2$".to_owned())
		);
	}
}
