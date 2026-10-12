use crate::{
	courses::CourseId,
	shared::{Entity, Id},
};

use bon::Builder;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

pub type QuestionId = Id<Question>;
pub type QuestionBankId = Id<QuestionBank>;

#[derive(Clone, Debug, Serialize, Deserialize, FromRow, Builder, PartialEq, Eq, Hash)]
pub struct QuestionBank {
	#[builder(default = QuestionBankId::new())]
	pub id: QuestionBankId,
	pub course_id: CourseId,
	pub name: String,
	pub created_at: DateTime<Utc>,
	pub deleted_at: Option<DateTime<Utc>>,
}

impl Entity for QuestionBank {
	fn key_name() -> &'static str {
		"question_bank"
	}
}

#[derive(Clone, Debug, Serialize, Deserialize, FromRow, Builder, PartialEq, Hash, Eq)]
pub struct Question {
	#[builder(default = QuestionId::new())]
	pub id: QuestionId,
	#[serde(skip_serializing)]
	pub bank_id: QuestionBankId,
	#[serde(skip_serializing)]
	pub position: i16,
	pub prompt: String,
	pub options: Vec<String>,
	pub answer_index: i16,
	/// Optional Markdown/LaTeX explanation shown to students in their results.
	pub justification: Option<String>,
	#[serde(skip_serializing)]
	pub created_at: DateTime<Utc>,
	#[serde(skip_serializing)]
	pub deleted_at: Option<DateTime<Utc>>,
}

impl Entity for Question {
	fn key_name() -> &'static str {
		"question"
	}
}

#[derive(Clone, Debug, Serialize, Deserialize, FromRow)]
pub struct QuestionView {
	pub id: QuestionId,
	pub prompt: String,
	pub options: Vec<String>,
}

impl From<Question> for QuestionView {
	fn from(question: Question) -> Self {
		Self {
			id: question.id,
			prompt: question.prompt,
			options: question.options,
		}
	}
}
