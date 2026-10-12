use std::sync::Arc;

use crate::banks::{QuestionBankId, QuestionId};
use crate::courses::CourseId;
use crate::quizzes::{Quiz, QuizId};
use crate::shared::{AppResult, Database, Tx};

use chrono::Utc;
use sword::prelude::*;

#[injectable]
pub struct QuizRepository {
	db: Arc<Database>,
}

impl QuizRepository {
	pub async fn find_by_id(&self, id: &QuizId) -> AppResult<Option<Quiz>> {
		let quiz = sqlx::query_as::<_, Quiz>(
			"SELECT * FROM quizzes
            WHERE id = $1 AND deleted_at IS NULL",
		)
		.bind(id)
		.fetch_optional(self.db.get_pool())
		.await?;

		Ok(quiz)
	}

	pub async fn find_by_code(&self, code: &str) -> AppResult<Option<Quiz>> {
		let quiz = sqlx::query_as::<_, Quiz>(
			"SELECT * FROM quizzes WHERE join_code = $1 AND deleted_at IS NULL",
		)
		.bind(code)
		.fetch_optional(self.db.get_pool())
		.await?;

		Ok(quiz)
	}

	pub async fn list_by_course(&self, course_id: &CourseId) -> AppResult<Vec<Quiz>> {
		let quizzes = sqlx::query_as::<_, Quiz>(
			"SELECT * FROM quizzes
             WHERE course_id = $1 AND deleted_at IS NULL
             ORDER BY created_at DESC",
		)
		.bind(course_id)
		.fetch_all(self.db.get_pool())
		.await?;

		Ok(quizzes)
	}

	pub async fn save(&self, tx: &mut Tx<'_>, quiz: &Quiz) -> AppResult<Quiz> {
		let quiz = sqlx::query_as::<_, Quiz>(
			"INSERT INTO quizzes (
                id,
                course_id,
                title,
                kind,
                join_code,
                question_count,
                max_score,
                certainty_table,
                attempt_duration_minutes,
                starts_at,
                results_published_at,
                results_multiple_views,
                created_at,
                deleted_at
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14)
            ON CONFLICT (id)
            DO UPDATE SET
                course_id = EXCLUDED.course_id,
                title = EXCLUDED.title,
                kind = EXCLUDED.kind,
                join_code = EXCLUDED.join_code,
                question_count = EXCLUDED.question_count,
                max_score = EXCLUDED.max_score,
                certainty_table = EXCLUDED.certainty_table,
                attempt_duration_minutes = EXCLUDED.attempt_duration_minutes,
                starts_at = EXCLUDED.starts_at,
                results_published_at = EXCLUDED.results_published_at,
                results_multiple_views = EXCLUDED.results_multiple_views,
                created_at = EXCLUDED.created_at,
                deleted_at = EXCLUDED.deleted_at
            RETURNING *",
		)
		.bind(quiz.id)
		.bind(quiz.course_id)
		.bind(&quiz.title)
		.bind(quiz.kind)
		.bind(&quiz.join_code)
		.bind(quiz.question_count)
		.bind(quiz.max_score)
		.bind(&quiz.certainty_table)
		.bind(quiz.attempt_duration_minutes)
		.bind(quiz.starts_at)
		.bind(quiz.results_published_at)
		.bind(quiz.results_multiple_views)
		.bind(quiz.created_at)
		.bind(quiz.deleted_at)
		.fetch_one(&mut **tx)
		.await?;

		Ok(quiz)
	}

	pub async fn publish_results(&self, quiz_id: &QuizId) -> AppResult<bool> {
		let now = Utc::now();

		let result = sqlx::query(
			"UPDATE quizzes
             SET results_published_at = COALESCE(results_published_at, $2)
             WHERE id = $1 AND deleted_at IS NULL",
		)
		.bind(quiz_id)
		.bind(now)
		.execute(self.db.get_pool())
		.await?;

		Ok(result.rows_affected() > 0)
	}

	pub async fn delete_by_id(&self, quiz_id: &QuizId) -> AppResult<bool> {
		let result = sqlx::query(
			"UPDATE quizzes SET deleted_at = $2
             WHERE id = $1 AND deleted_at IS NULL",
		)
		.bind(quiz_id)
		.bind(Utc::now())
		.execute(self.db.get_pool())
		.await?;

		Ok(result.rows_affected() > 0)
	}

	pub async fn set_bank_links(
		&self,
		tx: &mut Tx<'_>,
		quiz_id: &QuizId,
		bank_ids: &[QuestionBankId],
	) -> AppResult<()> {
		sqlx::query("DELETE FROM quiz_question_banks WHERE quiz_id = $1")
			.bind(quiz_id)
			.execute(&mut **tx)
			.await?;

		let mut query_builder =
			sqlx::QueryBuilder::new("INSERT INTO quiz_question_banks (quiz_id, question_bank_id) ");

		query_builder.push_values(bank_ids.iter(), |mut b, bank_id| {
			b.push_bind(quiz_id).push_bind(bank_id);
		});

		query_builder.build().execute(&mut **tx).await?;

		Ok(())
	}

	pub async fn set_quiz_questions(
		&self,
		tx: &mut Tx<'_>,
		quiz_id: &QuizId,
		question_ids: &[QuestionId],
	) -> AppResult<()> {
		sqlx::query("DELETE FROM quiz_questions WHERE quiz_id = $1")
			.bind(quiz_id)
			.execute(&mut **tx)
			.await?;

		if question_ids.is_empty() {
			return Ok(());
		}

		let mut query_builder =
			sqlx::QueryBuilder::new("INSERT INTO quiz_questions (quiz_id, question_id) ");

		query_builder.push_values(question_ids.iter(), |mut b, question_id| {
			b.push_bind(quiz_id).push_bind(question_id);
		});

		query_builder.build().execute(&mut **tx).await?;

		Ok(())
	}
}
