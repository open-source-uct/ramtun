import * as v from "valibot"

const questionInputSchema = v.object({
	prompt: v.pipe(
		v.string(),
		v.minLength(1, "Prompt debe tener entre 1 y 5000 caracteres."),
		v.maxLength(5000, "Prompt debe tener entre 1 y 5000 caracteres.")
	),
	options: v.pipe(
		v.array(v.string()),
		v.minLength(2, "Debe haber entre 2 y 5 opciones."),
		v.maxLength(5, "Debe haber entre 2 y 5 opciones.")
	),
	answerIndex: v.pipe(
		v.number(),
		v.integer("answerIndex debe ser un entero válido."),
		v.minValue(0, "answerIndex no puede ser negativo.")
	),
	justification: v.optional(
		v.pipe(
			v.string(),
			v.maxLength(5000, "La justificación no puede superar los 5000 caracteres.")
		)
	),
})

const questionSchema = v.pipe(
	questionInputSchema,
	v.check(
		(q) => q.answerIndex < q.options.length,
		"answerIndex debe ser un índice válido de las opciones."
	)
)

export const bankQuestionsSchema = v.pipe(
	v.array(questionSchema, "El archivo debe contener un array de preguntas."),
	v.minLength(1, "Debe haber al menos 1 pregunta."),
	v.maxLength(500, "Debe haber entre 1 y 500 preguntas.")
)

export type QuestionInput = v.InferInput<typeof questionInputSchema>

export type Question = {
	id: string
	prompt: string
	options: string[]
	justification?: string | null
	answer_index?: number
	answerIndex?: number
}

export type QuestionBank = {
	id: string
	course_id?: string
	courseId?: string
	name: string
	questions: Question[]
	created_at?: string
	createdAt?: string
	deleted_at?: string | null
	deletedAt?: string | null
}

export type CreateQuestionBankInput = {
	courseId: string
	name: string
	questions: {
		prompt: string
		options: string[]
		answerIndex: number
		justification?: string
	}[]
}

export type UpdateQuestionBankInput = {
	name?: string
	questions: {
		prompt: string
		options: string[]
		justification?: string | null
	}[]
}
