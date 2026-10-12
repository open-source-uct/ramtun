use chrono::{DateTime, Utc};
use serde::Serialize;
use sqlx::FromRow;

use crate::{
	attempts::{Attempt, AttemptAnswer, AttemptId},
	banks::{QuestionId, QuestionView},
	quizzes::{CertaintyLevel, QuizId, QuizKind},
	users::UserId,
};

#[derive(Debug, Clone, Serialize, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct AttemptListItemView {
	pub attempt_id: AttemptId,
	pub student_id: UserId,
	pub user_name: String,
	pub quiz_id: QuizId,
	pub started_at: DateTime<Utc>,
	pub expires_at: DateTime<Utc>,
	pub submitted_at: Option<DateTime<Utc>>,
	pub results_viewed_at: Option<DateTime<Utc>>,
	pub score: Option<i16>,
	pub grade: Option<f64>,
	pub warning_count: i64,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AttemptView {
	pub attempt_id: AttemptId,
	pub quiz_id: QuizId,
	pub kind: QuizKind,
	pub title: String,
	pub started_at: DateTime<Utc>,
	pub expires_at: DateTime<Utc>,
	pub submitted_at: Option<DateTime<Utc>>,
	pub results_viewed_at: Option<DateTime<Utc>>,
	pub questions: Vec<QuestionView>,
	pub answers: Vec<SavedAnswerView>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SavedAnswerView {
	pub question_id: QuestionId,
	pub answer_index: i16,
	pub certainty_level: Option<CertaintyLevel>,
}

impl From<AttemptAnswer> for SavedAnswerView {
	fn from(value: AttemptAnswer) -> Self {
		Self {
			question_id: value.question_id,
			answer_index: value.answer_index,
			certainty_level: value.certainty_level,
		}
	}
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AttemptSubmitView {
	pub attempt_id: AttemptId,
	pub quiz_id: QuizId,
	pub submitted_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct QuestionResultView {
	pub question_id: QuestionId,
	pub question: String,
	pub options: Vec<String>,
	pub answer_index: Option<i16>,
	pub correct_answer_index: i16,
	pub justification: Option<String>,
	pub certainty_level: Option<CertaintyLevel>,
	pub is_correct: bool,
	pub awarded_points: i16,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AttemptResultView {
	pub attempt_id: AttemptId,
	pub student_id: UserId,
	pub user_name: String,
	pub quiz_id: QuizId,
	pub submitted_at: DateTime<Utc>,
	pub grade: f64,
	pub score: i16,
	pub max_score: i16,
	pub results_viewed_at: Option<DateTime<Utc>>,
	pub questions: Vec<QuestionResultView>,
}

impl From<Attempt> for AttemptListItemView {
	fn from(value: Attempt) -> Self {
		Self {
			attempt_id: value.id,
			student_id: value.student_id,
			user_name: value.student_id.to_string(),
			quiz_id: value.quiz_id,
			started_at: value.started_at,
			expires_at: value.expires_at,
			submitted_at: value.submitted_at,
			results_viewed_at: value.results_viewed_at,
			score: value.score,
			grade: value.grade,
			warning_count: 0,
		}
	}
}

impl
	From<(
		Attempt,
		Vec<QuestionView>,
		QuizKind,
		String,
		Vec<SavedAnswerView>,
	)> for AttemptView
{
	fn from(
		value: (
			Attempt,
			Vec<QuestionView>,
			QuizKind,
			String,
			Vec<SavedAnswerView>,
		),
	) -> Self {
		let (attempt, questions, kind, title, answers) = value;
		Self {
			attempt_id: attempt.id,
			quiz_id: attempt.quiz_id,
			kind,
			title,
			started_at: attempt.started_at,
			expires_at: attempt.expires_at,
			submitted_at: attempt.submitted_at,
			results_viewed_at: attempt.results_viewed_at,
			questions,
			answers,
		}
	}
}
