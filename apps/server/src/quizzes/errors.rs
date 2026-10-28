use sword::web::HttpError;
use thiserror::Error;

use crate::{banks::QuestionId, quizzes::QuizId};

#[derive(Debug, Error, HttpError)]
pub enum QuizError {
	#[http(code = 404, message = "No se encontró el quiz solicitado.")]
	#[error("Quiz not found: {0}")]
	NotFound(QuizId),

	#[http(code = 400, message = "El ID del quiz proporcionado no es válido.")]
	#[error("Invalid quiz ID")]
	InvalidId,

	#[http(code = 400, message = "El código de acceso del quiz no es válido.")]
	#[error("Invalid quiz join code")]
	InvalidCode,

	#[http(code = 403, message = "No tienes acceso a este quiz.")]
	#[error("Forbidden quiz access")]
	Forbidden,

	#[http(
		code = 403,
		message = "Solo el dueño del quiz puede gestionar colaboradores."
	)]
	#[error("Only the owner can manage collaborators")]
	OnlyOwnerCanManageCollaborators,

	#[http(
		code = 409,
		message = "El colaborador ya está registrado en este quiz."
	)]
	#[error("Collaborator already exists")]
	CollaboratorAlreadyExists,

	#[http(code = 404, message = "No se encontró el colaborador para este quiz.")]
	#[error("Collaborator not found")]
	CollaboratorNotFound,

	#[http(
		code = 400,
		message = "Solo asistentes y docentes pueden ser colaboradores del quiz."
	)]
	#[error("Invalid collaborator role")]
	InvalidCollaboratorRole,

	#[http(
		code = 409,
		message = "Este quiz está cerrado y ya no acepta intentos."
	)]
	#[error("Quiz is closed")]
	Closed,

	#[http(
		code = 409,
		message = "Debes cerrar el quiz antes de publicar resultados."
	)]
	#[error("Quiz is not closed")]
	NotClosed,

	#[http(
		code = 409,
		message = "Los resultados del quiz aún no están publicados."
	)]
	#[error("Quiz results not published")]
	ResultsNotPublished,

	#[http(
		code = 409,
		message = "Este quiz ya tiene intentos o resultados publicados: solo puedes cambiar su título y si se pueden revisar los resultados más de una vez."
	)]
	#[error("Quiz has attempts and is locked")]
	LockedForAttempts,

	#[http(code = 400, message = "El modo de quiz proporcionado no es válido.")]
	#[error("Invalid quiz mode")]
	InvalidQuizMode,

	#[http(code = 400, message = "La fecha de inicio proporcionada no es válida.")]
	#[error("Invalid start time")]
	InvalidStartTime,

	#[http(
		code = 400,
		message = "La cantidad de preguntas no puede superar el total disponible en el banco."
	)]
	#[error("Invalid question count")]
	InvalidQuestionCount,

	#[http(
		code = 400,
		message = "Uno o más bancos no son válidos para el curso seleccionado."
	)]
	#[error("Banks are not valid for course")]
	InvalidBanksForCourse,

	#[http(code = 400, message = "Una o más preguntas no son válidas.")]
	#[error("One or more questions are invalid: {0}")]
	QuestionNotFound(QuestionId),
}
