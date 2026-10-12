use crate::{
	banks::{Question, QuestionBankId},
	quizzes::{Quiz, QuizId},
	shared::{AppResult, Database, Tx},
};

use chrono::Utc;
use std::sync::Arc;
use sword::prelude::*;

#[injectable]
pub struct QuestionRepository {
	db: Arc<Database>,
}

impl QuestionRepository {
	pub async fn list_current_by_bank(&self, bank_id: &QuestionBankId) -> AppResult<Vec<Question>> {
		let questions = sqlx::query_as::<_, Question>(
			"SELECT * FROM questions
             WHERE bank_id = $1 AND deleted_at IS NULL
             ORDER BY position ASC",
		)
		.bind(bank_id)
		.fetch_all(self.db.get_pool())
		.await?;

		Ok(questions)
	}

	pub async fn list_by_bank_ids(&self, bank_ids: &[QuestionBankId]) -> AppResult<Vec<Question>> {
		let rows = sqlx::query_as::<_, Question>(
			"SELECT q.* FROM questions q
             INNER JOIN question_banks qb ON qb.id = q.bank_id
             WHERE qb.id = ANY($1) AND q.deleted_at IS NULL AND qb.deleted_at IS NULL
             ORDER BY q.bank_id, q.position ASC",
		)
		.bind(bank_ids)
		.fetch_all(self.db.get_pool())
		.await?;

		Ok(rows)
	}

	/// Returns the questions pinned by a quiz, including archived versions,
	/// so attempts and published results keep their original content.
	pub async fn list_for_quiz(&self, quiz_id: &QuizId) -> AppResult<Vec<Question>> {
		let questions = sqlx::query_as::<_, Question>(
			"SELECT q.* FROM questions q
             INNER JOIN quiz_questions qq ON qq.question_id = q.id
             WHERE qq.quiz_id = $1",
		)
		.bind(quiz_id)
		.fetch_all(self.db.get_pool())
		.await?;

		Ok(questions)
	}

	pub async fn find_for_quiz(
		&self,
		quiz_id: &QuizId,
		question_id: &crate::banks::QuestionId,
	) -> AppResult<Option<Question>> {
		let question = sqlx::query_as::<_, Question>(
			"SELECT q.* FROM questions q
             INNER JOIN quiz_questions qq ON qq.question_id = q.id
             WHERE qq.quiz_id = $1 AND q.id = $2
             LIMIT 1",
		)
		.bind(quiz_id)
		.bind(question_id)
		.fetch_optional(self.db.get_pool())
		.await?;

		Ok(question)
	}

	pub async fn list_linked_quizzes(&self, bank_id: &QuestionBankId) -> AppResult<Vec<Quiz>> {
		let quizzes = sqlx::query_as::<_, Quiz>(
			"SELECT q.* FROM quizzes q
             INNER JOIN quiz_question_banks qqb ON qqb.quiz_id = q.id
             WHERE qqb.question_bank_id = $1 AND q.deleted_at IS NULL",
		)
		.bind(bank_id)
		.fetch_all(self.db.get_pool())
		.await?;

		Ok(quizzes)
	}

	/// Returns the current questions of every bank linked to a quiz.
	pub async fn list_questions_for_linked_banks(
		&self,
		quiz_id: &QuizId,
	) -> AppResult<Vec<Question>> {
		let rows = sqlx::query_as::<_, Question>(
			"SELECT q.* FROM questions q
             INNER JOIN quiz_question_banks qqb ON q.bank_id = qqb.question_bank_id
             WHERE qqb.quiz_id = $1 AND q.deleted_at IS NULL",
		)
		.bind(quiz_id)
		.fetch_all(self.db.get_pool())
		.await?;

		Ok(rows)
	}

	pub async fn list_questions_for_linked_banks_excluding(
		&self,
		quiz_id: &QuizId,
		excluded_bank_id: &QuestionBankId,
	) -> AppResult<Vec<Question>> {
		let rows = sqlx::query_as::<_, Question>(
			"SELECT q.* FROM questions q
             INNER JOIN quiz_question_banks qqb ON q.bank_id = qqb.question_bank_id
             WHERE qqb.quiz_id = $1
               AND q.deleted_at IS NULL
               AND q.bank_id <> $2",
		)
		.bind(quiz_id)
		.bind(excluded_bank_id)
		.fetch_all(self.db.get_pool())
		.await?;

		Ok(rows)
	}

	pub async fn insert_generation(
		&self,
		tx: &mut Tx<'_>,
		questions: &[Question],
	) -> AppResult<()> {
		let mut query_builder = sqlx::QueryBuilder::new(
			"INSERT INTO questions (id, bank_id, position, prompt, options, answer_index, justification, created_at, deleted_at) ",
		);

		query_builder.push_values(questions.iter(), |mut b, q| {
			b.push_bind(q.id)
				.push_bind(q.bank_id)
				.push_bind(q.position)
				.push_bind(&q.prompt)
				.push_bind(&q.options)
				.push_bind(q.answer_index)
				.push_bind(&q.justification)
				.push_bind(q.created_at)
				.push_bind(q.deleted_at);
		});

		query_builder.build().execute(&mut **tx).await?;

		Ok(())
	}

	/// Archives the current generation of a bank: rows still referenced by a
	/// quiz are soft-deleted (kept readable for published results) and rows no
	/// longer referenced are hard-deleted.
	pub async fn archive_or_purge_generation(
		&self,
		tx: &mut Tx<'_>,
		bank_id: &QuestionBankId,
	) -> AppResult<()> {
		sqlx::query(
			"DELETE FROM questions
             WHERE bank_id = $1
               AND deleted_at IS NULL
               AND id NOT IN (SELECT question_id FROM quiz_questions)",
		)
		.bind(bank_id)
		.execute(&mut **tx)
		.await?;

		sqlx::query(
			"UPDATE questions SET deleted_at = $2
             WHERE bank_id = $1
               AND deleted_at IS NULL
               AND id IN (SELECT question_id FROM quiz_questions)",
		)
		.bind(bank_id)
		.bind(Utc::now())
		.execute(&mut **tx)
		.await?;

		Ok(())
	}
}
