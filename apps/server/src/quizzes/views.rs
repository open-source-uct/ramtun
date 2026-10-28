use crate::{courses::Course, quizzes::*};

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QuizView {
	pub id: QuizId,
	pub title: String,
	pub kind: QuizKind,
	pub join_code: String,
	pub question_count: i16,
	pub certainty_table: Option<CertaintyTable>,
	pub attempt_duration_minutes: i16,
	pub starts_at: DateTime<Utc>,
	pub results_published_at: Option<DateTime<Utc>>,
	pub results_multiple_views: bool,
	pub created_at: DateTime<Utc>,

	pub course: Course,
}

impl From<(Quiz, Course)> for QuizView {
	fn from((quiz, course): (Quiz, Course)) -> Self {
		Self {
			id: quiz.id,
			title: quiz.title,
			kind: quiz.kind,
			join_code: quiz.join_code,
			question_count: quiz.question_count,
			certainty_table: quiz.certainty_table,
			attempt_duration_minutes: quiz.attempt_duration_minutes,
			starts_at: quiz.starts_at,
			results_published_at: quiz.results_published_at,
			results_multiple_views: quiz.results_multiple_views,
			created_at: quiz.created_at,
			course,
		}
	}
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JoinQuizPreviewView {
	pub id: QuizId,
	pub title: String,
	pub kind: QuizKind,
	pub question_count: i16,
	pub certainty_table: Option<CertaintyTable>,
	pub attempt_duration_minutes: i16,
	pub starts_at: DateTime<Utc>,
}

impl From<&Quiz> for JoinQuizPreviewView {
	fn from(quiz: &Quiz) -> Self {
		Self {
			id: quiz.id,
			title: quiz.title.clone(),
			kind: quiz.kind,
			question_count: quiz.question_count,
			certainty_table: quiz.certainty_table.clone(),
			attempt_duration_minutes: quiz.attempt_duration_minutes,
			starts_at: quiz.starts_at,
		}
	}
}
