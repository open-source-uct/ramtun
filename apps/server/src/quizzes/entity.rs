use crate::{
	courses::CourseId,
	shared::{Entity, Id},
};

use bon::Builder;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Type};

pub type QuizId = Id<Quiz>;

#[derive(Clone, Debug, Serialize, Deserialize, FromRow, Builder, Default)]
pub struct Quiz {
	#[builder(default = QuizId::new())]
	pub id: QuizId,
	pub course_id: CourseId,
	pub title: String,
	pub kind: QuizKind,
	pub join_code: String,
	pub question_count: i16,
	pub max_score: i16,
	pub certainty_table: Option<CertaintyTable>,
	pub attempt_duration_minutes: i16,
	pub starts_at: DateTime<Utc>,
	pub results_published_at: Option<DateTime<Utc>>,
	/// Lets students reopen their published results instead of viewing them once.
	#[builder(default)]
	pub results_multiple_views: bool,
	pub created_at: DateTime<Utc>,
	pub deleted_at: Option<DateTime<Utc>>,
}

impl Entity for Quiz {
	fn key_name() -> &'static str {
		"quiz"
	}
}

impl Quiz {
	/// Top score a student can reach: one point per question in traditional
	/// quizzes, the highest certainty reward per question otherwise.
	pub fn compute_max_score(
		kind: QuizKind,
		question_count: i16,
		certainty_table: Option<&CertaintyTable>,
	) -> i16 {
		match (kind, certainty_table) {
			(QuizKind::Certainty, Some(table)) => table.high.correct * question_count,
			_ => question_count,
		}
	}
}

#[derive(Clone, Debug, Serialize, Deserialize, Type, Default, PartialEq, Eq, Copy)]
#[sqlx(type_name = "quiz_kind", rename_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum QuizKind {
	Traditional,

	#[default]
	Certainty,
}

#[derive(Clone, Debug, Serialize, Deserialize, Type, PartialEq, Eq)]
#[sqlx(type_name = "certainty_score")]
pub struct CertaintyScore {
	pub correct: i16,
	pub incorrect: i16,
}

#[derive(Clone, Debug, Serialize, Deserialize, Type, PartialEq, Eq)]
#[sqlx(type_name = "certainty_table")]
pub struct CertaintyTable {
	pub low: CertaintyScore,
	pub medium: CertaintyScore,
	pub high: CertaintyScore,
}

#[derive(Clone, Debug, Serialize, Deserialize, Type, Copy, Default)]
#[sqlx(type_name = "certainty_level", rename_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum CertaintyLevel {
	#[default]
	Low,
	Medium,
	High,
}

impl Default for CertaintyTable {
	fn default() -> Self {
		CertaintyTable {
			low: CertaintyScore {
				correct: 1,
				incorrect: 0,
			},
			medium: CertaintyScore {
				correct: 2,
				incorrect: -2,
			},
			high: CertaintyScore {
				correct: 3,
				incorrect: -4,
			},
		}
	}
}
