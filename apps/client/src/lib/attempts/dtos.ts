import * as v from "valibot"

export const certaintyLevelDTOSchema = v.picklist(["low", "medium", "high"])
export type CertaintyLevel = v.InferOutput<typeof certaintyLevelDTOSchema>

export type AttemptQuestion = {
	id: string
	prompt: string
	options: string[]
}

export type SavedAnswer = {
	questionId: string
	answerIndex: number
	certaintyLevel: CertaintyLevel | null
}

export type AttemptView = {
	attemptId: string
	quizId: string
	kind: "traditional" | "certainty"
	title: string
	startedAt: string
	expiresAt: string
	submittedAt: string | null
	resultsViewedAt: string | null
	questions: AttemptQuestion[]
	answers: SavedAnswer[]
}

export type AttemptListItem = {
	attemptId: string
	studentId: string
	userName: string
	quizId: string
	startedAt: string
	expiresAt: string
	submittedAt: string | null
	resultsViewedAt: string | null
	score: number | null
	grade: number | null
	warningCount: number
}

export type AttemptSubmitView = {
	attemptId: string
	quizId: string
	submittedAt: string
}

export type QuestionResult = {
	questionId: string
	question: string
	options: string[]
	answerIndex: number | null
	correctAnswerIndex: number
	certaintyLevel: CertaintyLevel | null
	justification: string | null
	isCorrect: boolean
	awardedPoints: number
}

export type AttemptResult = {
	attemptId: string
	studentId: string
	userName: string
	quizId: string
	submittedAt: string
	grade: number
	score: number
	maxScore: number
	resultsViewedAt: string | null
	questions: QuestionResult[]
}

export type SaveAnswerInput = {
	answerIndex: number
	certaintyLevel: CertaintyLevel | null
	questionId: string
}

export type AnswerState = {
	answerIndex: number
	certaintyLevel: CertaintyLevel | null
}

export type WarningType =
	| "context_menu"
	| "copy_attempt"
	| "search_attempt"
	| "screenshot"
	| "alt_tab"

export type AttemptWarning = {
	id: string
	attemptId: string
	warningType: WarningType
	details: string
	sequenceNumber: number
	createdAt: string
}

export const WARNING_LABELS: Record<WarningType, string> = {
	context_menu: "Menú contextual",
	copy_attempt: "Intento de copia",
	search_attempt: "Búsqueda en página",
	screenshot: "Captura de pantalla",
	alt_tab: "Cambio de aplicación",
}

export type SeverityLevel = "leve" | "moderada" | "grave"

export const SEVERITY_GROUPS: SeverityLevel[] = ["leve", "moderada", "grave"]

export const WARNING_SEVERITY: Record<WarningType, SeverityLevel> = {
	context_menu: "leve",
	search_attempt: "leve",
	copy_attempt: "moderada",
	screenshot: "grave",
	alt_tab: "grave",
}
