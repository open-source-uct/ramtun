use crate::courses::CourseId;

use serde::{Deserialize, Serialize};
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
	pub questions: Option<Vec<QuestionInput>>,
}

fn validate_question(question: &QuestionInput) -> Result<(), ValidationError> {
	if question.answer_index < question.options.len() {
		return Ok(());
	}

	let mut err = ValidationError::new("invalid_answer_index");
	err.message = Some("answerIndex must be a valid option index".into());
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

#[cfg(test)]
mod tests {
	use super::{QuestionInput, UpdateQuestionBankDto};
	use validator::Validate;

	fn question(options: usize, answer_index: usize) -> QuestionInput {
		QuestionInput {
			prompt: "P".to_owned(),
			options: (0..options).map(|i| format!("opción {i}")).collect(),
			answer_index,
			justification: None,
		}
	}

	fn update_with(questions: Vec<QuestionInput>) -> UpdateQuestionBankDto {
		UpdateQuestionBankDto {
			name: None,
			questions: Some(questions),
		}
	}

	#[test]
	fn update_accepts_a_changed_correct_answer() {
		assert!(update_with(vec![question(3, 2)]).validate().is_ok());
	}

	#[test]
	fn update_rejects_a_correct_answer_outside_the_options() {
		assert!(update_with(vec![question(3, 3)]).validate().is_err());
	}

	#[test]
	fn update_rejects_questions_with_fewer_than_two_options() {
		assert!(update_with(vec![question(1, 0)]).validate().is_err());
	}

	#[test]
	fn update_rejects_an_empty_payload() {
		let dto = UpdateQuestionBankDto {
			name: None,
			questions: None,
		};

		assert!(dto.validate().is_err());
	}
}
