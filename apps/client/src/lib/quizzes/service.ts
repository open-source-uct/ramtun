import { Quiz } from "./entity"
import { http } from "$lib/shared/http/client"

import type { CreateQuizDTO, QuizDTO, UpdateQuizDTO } from "$lib/quizzes/dtos"

class QuizzesService {
	public list(courseId: string): Promise<Quiz[]> {
		const quizzes = http.request<QuizDTO[]>({
			method: "GET",
			url: `/quizzes/course/${courseId}`,
		})

		return quizzes.then((quizzes) => quizzes.map((quiz) => Quiz.fromDTO(quiz)))
	}

	public create(data: CreateQuizDTO): Promise<Quiz> {
		const quiz = http.request<QuizDTO>({
			method: "POST",
			url: "/quizzes",
			data,
		})

		return quiz.then((quiz) => Quiz.fromDTO(quiz))
	}

	public update(quizId: string, data: UpdateQuizDTO): Promise<Quiz> {
		const quiz = http.request<QuizDTO>({
			method: "PATCH",
			url: `/quizzes/${quizId}`,
			data,
		})

		return quiz.then((quiz) => Quiz.fromDTO(quiz))
	}

	public remove(quizId: string): Promise<void> {
		return http.request<void>({
			method: "DELETE",
			url: `/quizzes/${quizId}`,
		})
	}

	public closeAndPublish(quizId: string): Promise<void> {
		return http.request<void>({
			method: "POST",
			url: `/quizzes/${quizId}/close-and-publish`,
		})
	}

	public findOne(quizId: string): Promise<Quiz> {
		const quiz = http.request<QuizDTO>({
			method: "GET",
			url: `/quizzes/${quizId}`,
		})

		return quiz.then((quiz) => Quiz.fromDTO(quiz))
	}

	public joinByCode(joinCode: string): Promise<Quiz> {
		const quiz = http.request<QuizDTO>({
			method: "POST",
			url: `/quizzes/join/${joinCode}`,
		})

		return quiz.then((quiz) => Quiz.fromDTO(quiz))
	}
}

export const quizzesService = new QuizzesService()
