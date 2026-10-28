use std::sync::Arc;

use sqlx::{Postgres, QueryBuilder};
use sword::prelude::*;

use crate::{
	attempts::{
		Attempt, AttemptAnswer, AttemptFilter, AttemptId, AttemptListItemView, AttemptWarning,
	},
	quizzes::QuizId,
	shared::{AppResult, Database},
	users::UserId,
};

use chrono::Utc;

#[injectable]
pub struct AttemptRepository {
	db: Arc<Database>,
}

impl AttemptRepository {
	pub async fn find_by_quiz_and_student(
		&self,
		quiz_id: &QuizId,
		student_id: &UserId,
	) -> AppResult<Option<Attempt>> {
		let attempt = sqlx::query_as::<_, Attempt>(
			"SELECT * FROM attempts
             WHERE quiz_id = $1 AND student_id = $2 AND deleted_at IS NULL",
		)
		.bind(quiz_id)
		.bind(student_id)
		.fetch_optional(self.db.get_pool())
		.await?;

		Ok(attempt)
	}

	pub async fn exists_for_quiz(&self, quiz_id: &QuizId) -> AppResult<bool> {
		let exists = sqlx::query_scalar::<_, bool>(
			"SELECT EXISTS (
                SELECT 1 FROM attempts WHERE quiz_id = $1 AND deleted_at IS NULL
            )",
		)
		.bind(quiz_id)
		.fetch_one(self.db.get_pool())
		.await?;

		Ok(exists)
	}

	pub async fn find_by_id(&self, attempt_id: &AttemptId) -> AppResult<Option<Attempt>> {
		let attempt = sqlx::query_as::<_, Attempt>(
			"SELECT * FROM attempts WHERE id = $1 AND deleted_at IS NULL",
		)
		.bind(attempt_id)
		.fetch_optional(self.db.get_pool())
		.await?;

		Ok(attempt)
	}

	pub async fn find_active_by_user(&self, student_id: &UserId) -> AppResult<Option<Attempt>> {
		let attempt = sqlx::query_as::<_, Attempt>(
			"SELECT * FROM attempts
             WHERE student_id = $1
             AND submitted_at IS NULL
             AND expires_at > $2
             AND deleted_at IS NULL
             ORDER BY started_at DESC
             LIMIT 1",
		)
		.bind(student_id)
		.bind(Utc::now())
		.fetch_optional(self.db.get_pool())
		.await?;

		Ok(attempt)
	}

	pub async fn list_attempts(&self, filter: AttemptFilter) -> AppResult<Vec<Attempt>> {
		let mut query =
			QueryBuilder::<Postgres>::new("SELECT * FROM attempts WHERE deleted_at IS NULL");

		query.push(" AND quiz_id = ").push_bind(filter.quiz_id);

		if let Some(student_id) = filter.student_id {
			query.push(" AND student_id = ").push_bind(student_id);
		}

		let attempts = query
			.build_query_as::<Attempt>()
			.fetch_all(self.db.get_pool())
			.await?;

		Ok(attempts)
	}

	pub async fn list_attempts_managed(
		&self,
		filter: AttemptFilter,
	) -> AppResult<Vec<AttemptListItemView>> {
		let mut query = QueryBuilder::<Postgres>::new(
            "SELECT
                a.id AS attempt_id,
                a.student_id,
                u.name AS user_name,
                a.quiz_id,
                a.started_at,
                a.expires_at,
                a.submitted_at,
                a.results_viewed_at,
                a.score,
                a.grade,
                (SELECT COUNT(*)::BIGINT FROM attempt_warnings aw WHERE aw.attempt_id = a.id) AS warning_count
             FROM attempts a
             JOIN users u ON u.id = a.student_id
             WHERE a.deleted_at IS NULL",
        );

		query.push(" AND a.quiz_id = ").push_bind(filter.quiz_id);

		if let Some(student_id) = filter.student_id {
			query.push(" AND a.student_id = ").push_bind(student_id);
		}

		query.push(" ORDER BY a.started_at DESC");

		let attempts = query
			.build_query_as::<AttemptListItemView>()
			.fetch_all(self.db.get_pool())
			.await?;

		Ok(attempts)
	}

	pub async fn save(&self, attempt: &Attempt) -> AppResult<()> {
		sqlx::query(
			"INSERT INTO attempts (id, student_id, quiz_id, question_order, score, grade, started_at, expires_at, submitted_at, results_viewed_at, deleted_at)
			VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
			ON CONFLICT (id) DO UPDATE SET
				score = EXCLUDED.score,
				grade = EXCLUDED.grade,
				submitted_at = EXCLUDED.submitted_at,
				results_viewed_at = EXCLUDED.results_viewed_at,
				deleted_at = EXCLUDED.deleted_at
			"
		)
		.bind(attempt.id)
		.bind(attempt.student_id)
		.bind(attempt.quiz_id)
		.bind(&attempt.question_order)
		.bind(attempt.score)
		.bind(attempt.grade)
		.bind(attempt.started_at)
		.bind(attempt.expires_at)
		.bind(attempt.submitted_at)
		.bind(attempt.results_viewed_at)
		.bind(attempt.deleted_at)
		.execute(self.db.get_pool())
		.await?;

		Ok(())
	}

	pub async fn list_attempt_answers(
		&self,
		attempt_id: &AttemptId,
	) -> AppResult<Vec<AttemptAnswer>> {
		let answers = sqlx::query_as::<_, AttemptAnswer>(
			"SELECT * FROM attempt_answers WHERE attempt_id = $1",
		)
		.bind(attempt_id)
		.fetch_all(self.db.get_pool())
		.await?;

		Ok(answers)
	}

	pub async fn upsert_attempt_answer(&self, answer: &AttemptAnswer) -> AppResult<()> {
		sqlx::query(
			"INSERT INTO attempt_answers (
                attempt_id,
                question_id,
                answer_index,
                certainty_level,
                is_correct,
                awarded_points
             )
             VALUES ($1, $2, $3, $4, $5, $6)
             ON CONFLICT (attempt_id, question_id)
             DO UPDATE SET
                answer_index = EXCLUDED.answer_index,
                certainty_level = EXCLUDED.certainty_level,
                is_correct = EXCLUDED.is_correct,
                awarded_points = EXCLUDED.awarded_points",
		)
		.bind(answer.attempt_id)
		.bind(answer.question_id)
		.bind(answer.answer_index)
		.bind(answer.certainty_level)
		.bind(answer.is_correct)
		.bind(answer.awarded_points)
		.execute(self.db.get_pool())
		.await?;

		Ok(())
	}

	pub async fn mark_results_viewed(&self, attempt_id: &AttemptId) -> AppResult<()> {
		sqlx::query(
			"UPDATE attempts SET results_viewed_at = COALESCE(results_viewed_at, NOW())
             WHERE id = $1 AND deleted_at IS NULL",
		)
		.bind(attempt_id)
		.execute(self.db.get_pool())
		.await?;

		Ok(())
	}

	pub async fn insert_warning(&self, warning: &AttemptWarning) -> AppResult<()> {
		sqlx::query(
            "INSERT INTO attempt_warnings (id, attempt_id, warning_type, details, sequence_number, created_at)
             VALUES ($1, $2, $3, $4, $5, $6)",
        )
        .bind(warning.id)
        .bind(warning.attempt_id)
        .bind(warning.warning_type)
        .bind(&warning.details)
        .bind(warning.sequence_number)
        .bind(warning.created_at)
        .execute(self.db.get_pool())
        .await?;

		Ok(())
	}

	pub async fn get_next_warning_sequence(&self, attempt_id: &AttemptId) -> AppResult<i16> {
		let row: Option<(i16,)> = sqlx::query_as(
			"SELECT (COALESCE(MAX(sequence_number), 0::SMALLINT) + 1)::SMALLINT
             FROM attempt_warnings
             WHERE attempt_id = $1",
		)
		.bind(attempt_id)
		.fetch_optional(self.db.get_pool())
		.await?;

		Ok(row.map(|r| r.0).unwrap_or(1))
	}

	pub async fn get_warnings_by_attempt(
		&self,
		attempt_id: &AttemptId,
	) -> AppResult<Vec<AttemptWarning>> {
		let warnings = sqlx::query_as::<_, AttemptWarning>(
			"SELECT * FROM attempt_warnings WHERE attempt_id = $1 ORDER BY sequence_number ASC",
		)
		.bind(attempt_id)
		.fetch_all(self.db.get_pool())
		.await?;

		Ok(warnings)
	}

	pub async fn get_warnings_by_quiz(&self, quiz_id: &QuizId) -> AppResult<Vec<AttemptWarning>> {
		let warnings = sqlx::query_as::<_, AttemptWarning>(
			"SELECT aw.* FROM attempt_warnings aw
             JOIN attempts a ON a.id = aw.attempt_id
             WHERE a.quiz_id = $1 AND a.deleted_at IS NULL
             ORDER BY aw.created_at DESC",
		)
		.bind(quiz_id)
		.fetch_all(self.db.get_pool())
		.await?;

		Ok(warnings)
	}
}
