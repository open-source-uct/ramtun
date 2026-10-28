use crate::auth::SessionCheck;
use crate::authz::{AuthzAction, AuthzGuard};
use crate::courses::CourseId;
use crate::quizzes::*;
use crate::shared::RequestExt;

use std::sync::Arc;
use sword::prelude::*;
use sword::web::*;

#[controller(kind = Controller::Web, path = "/quizzes")]
#[interceptor(SessionCheck)]
pub struct QuizController {
	service: Arc<QuizService>,
}

impl QuizController {
	#[get("/{quizId}")]
	#[interceptor(AuthzGuard, config = AuthzAction::QuizReadManaged)]
	pub async fn get_one(&self, req: Request) -> WebResult {
		let quiz_id = req.param::<QuizId>("quizId")?;
		let current_user = req.user().ok_or_else(JsonResponse::Unauthorized)?;

		let quiz = self.service.get_one(current_user, &quiz_id).await?;

		Ok(JsonResponse::Ok().data(quiz))
	}

	#[get("/course/{courseId}")]
	#[interceptor(AuthzGuard, config = AuthzAction::QuizListManaged)]
	pub async fn list_by_course(&self, req: Request) -> WebResult {
		let course_id = req.param::<CourseId>("courseId")?;
		let current_user = req.user().ok_or_else(JsonResponse::Unauthorized)?;

		let quizzes = self
			.service
			.list_by_course(current_user, &course_id)
			.await?;

		Ok(JsonResponse::Ok().data(quizzes))
	}

	#[post("/")]
	#[interceptor(AuthzGuard, config = AuthzAction::QuizCreate)]
	pub async fn create(&self, req: Request) -> WebResult {
		let current_user = req.user().ok_or_else(JsonResponse::Unauthorized)?;
		let input = req.body_validator::<CreateQuizDto>()?;

		let quiz = self.service.create(current_user, input).await?;

		Ok(JsonResponse::Created().data(quiz))
	}

	#[patch("/{quizId}")]
	#[interceptor(AuthzGuard, config = AuthzAction::QuizUpdateManaged)]
	pub async fn update(&self, req: Request) -> WebResult {
		let quiz_id = req.param::<QuizId>("quizId")?;
		let input = req.body_validator::<UpdateQuizDto>()?;
		let current_user = req.user().ok_or_else(JsonResponse::Unauthorized)?;

		let quiz = self.service.update(current_user, &quiz_id, input).await?;

		Ok(JsonResponse::Ok().data(quiz))
	}

	#[delete("/{quizId}")]
	#[interceptor(AuthzGuard, config = AuthzAction::QuizDeleteManaged)]
	pub async fn delete(&self, req: Request) -> WebResult {
		let quiz_id = req.param::<QuizId>("quizId")?;
		let current_user = req.user().ok_or_else(JsonResponse::Unauthorized)?;

		self.service.delete_quiz(current_user, &quiz_id).await?;

		Ok(JsonResponse::Ok().message("Quiz eliminado correctamente"))
	}

	#[post("/{quizId}/close-and-publish")]
	#[interceptor(AuthzGuard, config = AuthzAction::QuizCloseAndPublishManaged)]
	pub async fn close_and_publish(&self, req: Request) -> WebResult {
		let quiz_id = req.param::<QuizId>("quizId")?;
		let current_user = req.user().ok_or_else(JsonResponse::Unauthorized)?;

		self.service
			.close_and_publish_results(current_user, &quiz_id)
			.await?;

		Ok(JsonResponse::Ok().message("Quiz finalizado y resultados publicados"))
	}

	#[post("/join/{joinCode}")]
	#[interceptor(AuthzGuard, config = AuthzAction::QuizJoinByCode)]
	pub async fn join_by_code(&self, req: Request) -> WebResult {
		let code = req.param::<String>("joinCode")?;
		let preview = self.service.get_join_preview(&code).await?;

		Ok(JsonResponse::Ok().data(preview))
	}
}
