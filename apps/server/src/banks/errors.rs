use sword::web::HttpError;
use thiserror::Error;

#[derive(Debug, Error, HttpError)]
pub enum QuestionBankError {
	#[http(
		code = 404,
		message = "No se encontró el banco de preguntas solicitado."
	)]
	#[error("Question bank not found: {0}")]
	NotFound(String),

	#[http(
		code = 400,
		message = "El ID del banco de preguntas proporcionado no es válido."
	)]
	#[error("Invalid question bank ID")]
	InvalidId,

	#[http(code = 403, message = "No tienes acceso a este banco de preguntas.")]
	#[error("Forbidden question bank access")]
	Forbidden,

	#[http(
		code = 409,
		message = "Este banco está siendo usado por un quiz en curso y no se puede modificar."
	)]
	#[error("Question bank is locked by running quiz")]
	LockedByRunningQuiz,

	#[http(
		code = 409,
		message = "La versión resultante del banco tiene menos preguntas que las requeridas por el quiz."
	)]
	#[error("Invalid question count after bank update")]
	InvalidQuestionCountAfterBankUpdate,
}
