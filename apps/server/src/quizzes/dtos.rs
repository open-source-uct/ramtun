use crate::{banks::QuestionBankId, courses::CourseId, quizzes::*};

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use validator::{Validate, ValidationError};

#[derive(Clone, Debug, Validate, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[validate(schema(function = "validate_create_schema", skip_on_field_errors = false))]
pub struct CreateQuizDto {
	pub course_id: CourseId,

	#[validate(length(
		min = 1,
		max = 100,
		message = "Title must be between 1 and 100 characters"
	))]
	pub title: String,

	pub kind: QuizKind,
	pub starts_at: String,

	#[validate(range(
		min = 1,
		max = 240,
		message = "Duration must be between 1 and 240 minutes"
	))]
	pub attempt_duration_minutes: i16,

	#[validate(range(
		min = 1,
		max = 100,
		message = "Question count must be between 1 and 100"
	))]
	pub question_count: i16,

	#[validate(length(min = 1, message = "At least one bank is required"))]
	pub bank_ids: Vec<QuestionBankId>,

	#[validate(nested)]
	pub certainty_config: Option<CertaintyTableDto>,

	#[serde(default)]
	pub results_multiple_views: bool,
}

/// Partial update: only the fields present are applied. The quiz kind and its
/// banks are fixed at creation.
#[derive(Clone, Debug, Validate, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[validate(schema(function = "validate_update_schema", skip_on_field_errors = false))]
pub struct UpdateQuizDto {
	#[validate(length(
		min = 1,
		max = 100,
		message = "Title must be between 1 and 100 characters"
	))]
	pub title: Option<String>,

	pub starts_at: Option<String>,

	#[validate(range(
		min = 1,
		max = 240,
		message = "Duration must be between 1 and 240 minutes"
	))]
	pub attempt_duration_minutes: Option<i16>,

	#[validate(range(
		min = 1,
		max = 100,
		message = "Question count must be between 1 and 100"
	))]
	pub question_count: Option<i16>,

	#[validate(nested)]
	pub certainty_config: Option<CertaintyTableDto>,

	pub results_multiple_views: Option<bool>,
}

#[derive(Clone, Debug, Validate, Serialize, Deserialize)]
pub struct CertaintyTableDto {
	#[validate(nested)]
	pub low: CertaintyScoreDto,

	#[validate(nested)]
	pub medium: CertaintyScoreDto,

	#[validate(nested)]
	pub high: CertaintyScoreDto,
}

#[derive(Clone, Debug, Validate, Serialize, Deserialize)]
pub struct CertaintyScoreDto {
	#[validate(range(
		min = 0,
		max = 100,
		message = "Correct values must be between 0 and 100"
	))]
	pub correct: i16,

	#[validate(range(
        min = -100,
        max = 0,
        message = "Incorrect values must be between -100 and 0"
    ))]
	pub incorrect: i16,
}

fn validate_create_schema(dto: &CreateQuizDto) -> Result<(), ValidationError> {
	validate_start_time(&dto.starts_at)?;
	validate_quiz_mode(dto)
}

fn validate_update_schema(dto: &UpdateQuizDto) -> Result<(), ValidationError> {
	if let Some(starts_at) = &dto.starts_at {
		DateTime::parse_from_rfc3339(starts_at).map_err(|_| {
			let mut err = ValidationError::new("invalid_start_time_format");
			err.message = Some("startsAt must be a valid RFC3339 datetime string".into());
			err
		})?;
	}

	let has_fields = dto.title.is_some()
		|| dto.starts_at.is_some()
		|| dto.attempt_duration_minutes.is_some()
		|| dto.question_count.is_some()
		|| dto.certainty_config.is_some()
		|| dto.results_multiple_views.is_some();

	if has_fields {
		return Ok(());
	}

	let mut err = ValidationError::new("empty_update");
	err.message = Some("At least one field must be provided".into());
	Err(err)
}

fn validate_start_time(start_time_utc: &str) -> Result<(), ValidationError> {
	if let Ok(start_time) = DateTime::parse_from_rfc3339(start_time_utc) {
		if start_time < Utc::now() {
			let mut err = ValidationError::new("start_time_in_past");
			err.message = Some("startTimeUtc cannot be in the past".into());
			return Err(err);
		}

		return Ok(());
	}

	let mut err = ValidationError::new("invalid_start_time_format");
	err.message = Some("startTimeUtc must be a valid RFC3339 datetime string".into());

	Err(err)
}

fn validate_quiz_mode(dto: &CreateQuizDto) -> Result<(), ValidationError> {
	let kind = &dto.kind;
	let has_certainty_config = dto.certainty_config.is_some();

	match (kind, has_certainty_config) {
		(QuizKind::Certainty, false) => {
			let mut err = ValidationError::new("certainty_config_required");
			err.message = Some("certaintyConfig is required for certainty quizzes".into());
			Err(err)
		}
		(QuizKind::Traditional, true) => {
			let mut err = ValidationError::new("certainty_config_not_allowed");
			err.message = Some("certaintyConfig is not allowed for traditional quizzes".into());
			Err(err)
		}
		(QuizKind::Certainty, true) | (QuizKind::Traditional, false) => Ok(()),
	}
}

impl From<CertaintyTableDto> for CertaintyTable {
	fn from(value: CertaintyTableDto) -> Self {
		Self {
			low: value.low.into(),
			medium: value.medium.into(),
			high: value.high.into(),
		}
	}
}

impl From<CertaintyScoreDto> for CertaintyScore {
	fn from(value: CertaintyScoreDto) -> Self {
		Self {
			correct: value.correct,
			incorrect: value.incorrect,
		}
	}
}

#[cfg(test)]
mod tests {
	use super::UpdateQuizDto;
	use validator::Validate;

	fn parse(json: &str) -> UpdateQuizDto {
		serde_json::from_str(json).expect("valid quiz payload")
	}

	#[test]
	fn update_accepts_a_single_field() {
		assert!(parse(r#"{"title": "Control 2"}"#).validate().is_ok());
	}

	#[test]
	fn update_rejects_an_empty_payload() {
		assert!(parse("{}").validate().is_err());
	}

	#[test]
	fn update_rejects_an_invalid_start_time() {
		assert!(parse(r#"{"startsAt": "mañana"}"#).validate().is_err());
	}

	#[test]
	fn update_rejects_an_out_of_range_duration() {
		assert!(
			parse(r#"{"attemptDurationMinutes": 241}"#)
				.validate()
				.is_err()
		);
	}
}
