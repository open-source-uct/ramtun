import * as v from "valibot"

import type { CourseDTO } from "$lib/courses/dtos"

// Quiz view DTOs ------------------------------------------------------

export interface QuizDTO {
	id: string
	title: string
	kind: QuizKindDTO
	joinCode: string
	questionCount: number
	certaintyTable: CertaintyTableDTO | null
	attemptDurationMinutes: number
	startsAt: string
	resultsPublishedAt: string | null
	resultsMultipleViews: boolean
	createdAt: string
	course: Omit<CourseDTO, "members">
}

export const quizKindDTOSchema = v.picklist(["traditional", "certainty"])
export type QuizKindDTO = v.InferOutput<typeof quizKindDTOSchema>

export const certaintyScoreDTOSchema = v.object({
	correct: v.number(),
	incorrect: v.number(),
})

export type CertaintyScoreDTO = v.InferOutput<typeof certaintyScoreDTOSchema>

export const certaintyTableDTOSchema = v.object({
	low: certaintyScoreDTOSchema,
	medium: certaintyScoreDTOSchema,
	high: certaintyScoreDTOSchema,
})

export type CertaintyTableDTO = v.InferOutput<typeof certaintyTableDTOSchema>

// Create Quiz DTO ------------------------------------------------------

export const createQuizDTOSchema = v.object({
	courseId: v.string("El id del curso es obligatorio."),
	title: v.pipe(
		v.string(),
		v.trim(),
		v.minLength(1, "El título es obligatorio."),
		v.maxLength(100, "Máximo 100 caracteres.")
	),
	kind: quizKindDTOSchema,
	startsAt: v.pipe(
		v.string(),
		v.minLength(1, "La fecha de inicio es obligatoria."),
		v.check((value) => {
			const d = new Date(value)
			return !Number.isNaN(d.getTime()) && d > new Date()
		}, "La fecha de inicio debe estar en el futuro.")
	),
	attemptDurationMinutes: v.pipe(
		v.string(),
		v.toNumber(),
		v.minValue(1, "Mínimo 1 minuto."),
		v.maxValue(240, "Máximo 240 minutos.")
	),
	questionCount: v.pipe(
		v.string(),
		v.toNumber(),
		v.minValue(1, "Mínimo 1 pregunta."),
		v.maxValue(100, "Máximo 100 preguntas.")
	),
	bankIds: v.pipe(v.array(v.string()), v.minLength(1, "Selecciona al menos un banco.")),
	certaintyConfig: v.nullable(certaintyTableDTOSchema),
	resultsMultipleViews: v.boolean(),
})

export type CreateQuizDTOSchema = typeof createQuizDTOSchema
export type CreateQuizDTO = v.InferOutput<typeof createQuizDTOSchema>

// Update Quiz DTO ------------------------------------------------------

/** Solo viajan los campos que cambiaron: el servidor bloquea los estructurales si ya hay intentos. */
export type UpdateQuizDTO = {
	title?: string
	startsAt?: string
	attemptDurationMinutes?: number
	questionCount?: number
	certaintyConfig?: CertaintyTableDTO
	resultsMultipleViews?: boolean
}

// Join Quiz DTO ------------------------------------------------------

export const joinCodeDTOSchema = v.object({
	joinCode: v.nonOptional(
		v.pipe(
			v.string(),
			v.trim(),
			v.minLength(4, (info) =>
				info.input === "" ? "Ingresa un código." : "El código es demasiado corto."
			),
			v.maxLength(32, "El código es demasiado largo.")
		),
		"El código es obligatorio."
	),
})

export type JoinCodeDTOSchema = typeof joinCodeDTOSchema
