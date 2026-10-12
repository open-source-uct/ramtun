use crate::courses::CourseId;

use serde::{Deserialize, Deserializer, Serialize, Serializer};
use validator::{Validate, ValidationError};

#[derive(Clone, Debug, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
#[validate(schema(function = "validate_question", skip_on_field_errors = false))]
pub struct QuestionInput {
	#[validate(length(
		min = 1,
		max = 5000,
		message = "Prompt must be between 1 and 5000 characters"
	))]
	pub prompt: String,

	#[validate(length(min = 2, max = 5, message = "There must be between 2 and 5 options"))]
	pub options: Vec<String>,
	pub answer_index: usize,
	#[serde(default)]
	#[validate(length(max = 5000, message = "Justification must be at most 5000 characters"))]
	pub justification: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub enum PatchField<T> {
	#[default]
	Missing,
	Clear,
	Set(T),
}

impl<T> PatchField<T> {
	fn is_missing(&self) -> bool {
		matches!(self, Self::Missing)
	}
}

impl<'de, T> Deserialize<'de> for PatchField<T>
where
	T: Deserialize<'de>,
{
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where
		D: Deserializer<'de>,
	{
		Option::<T>::deserialize(deserializer).map(|value| match value {
			Some(value) => Self::Set(value),
			None => Self::Clear,
		})
	}
}

impl<T> Serialize for PatchField<T>
where
	T: Serialize,
{
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: Serializer,
	{
		match self {
			Self::Missing | Self::Clear => serializer.serialize_none(),
			Self::Set(value) => value.serialize(serializer),
		}
	}
}

#[derive(Clone, Debug, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct CreateQuestionBankDto {
	pub course_id: CourseId,
	#[validate(length(
		min = 1,
		max = 120,
		message = "Name must be between 1 and 120 characters"
	))]
	pub name: String,
	#[validate(length(
		min = 1,
		max = 500,
		message = "There must be at least 1 and at most 500 questions"
	))]
	#[validate(nested)]
	pub questions: Vec<QuestionInput>,
}

#[derive(Clone, Debug, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct UpdateQuestionInput {
	#[validate(length(
		min = 1,
		max = 5000,
		message = "Prompt must be between 1 and 5000 characters"
	))]
	pub prompt: String,

	#[validate(length(min = 2, max = 5, message = "There must be between 2 and 5 options"))]
	pub options: Vec<String>,
	#[serde(default, skip_serializing_if = "PatchField::is_missing")]
	#[validate(custom(function = "validate_justification_update"))]
	pub justification: PatchField<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
#[validate(schema(function = "validate_update_has_fields", skip_on_field_errors = false))]
pub struct UpdateQuestionBankDto {
	#[validate(length(
		min = 1,
		max = 120,
		message = "Name must be between 1 and 120 characters"
	))]
	pub name: Option<String>,
	#[validate(length(
		min = 1,
		max = 500,
		message = "There must be at least 1 and at most 500 questions"
	))]
	#[validate(nested)]
	pub questions: Option<Vec<UpdateQuestionInput>>,
}

fn validate_question(question: &QuestionInput) -> Result<(), ValidationError> {
	if question.answer_index < question.options.len() {
		return Ok(());
	}

	let mut err = ValidationError::new("invalid_answer_index");
	err.message = Some("answerIndex must be a valid option index".into());
	Err(err)
}

fn validate_justification_update(value: &PatchField<String>) -> Result<(), ValidationError> {
	let PatchField::Set(text) = value else {
		return Ok(());
	};

	if text.chars().count() <= 5000 {
		return Ok(());
	}

	let mut err = ValidationError::new("length");
	err.message = Some("Justification must be at most 5000 characters".into());
	Err(err)
}

fn validate_update_has_fields(dto: &UpdateQuestionBankDto) -> Result<(), ValidationError> {
	if dto.name.is_some() || dto.questions.is_some() {
		return Ok(());
	}

	let mut err = ValidationError::new("empty_update");
	err.message = Some("At least one field must be provided".into());
	Err(err)
}
