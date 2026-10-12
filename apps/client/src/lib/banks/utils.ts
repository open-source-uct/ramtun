import type { QuestionBank } from "$lib/banks/banks.dtos"
import { bankToMarkdown } from "$lib/banks/markdown"

type QuestionExport = {
	prompt: string
	options: string[]
	answerIndex: number
	justification?: string
}

const sanitizeFileName = (name: string) =>
	name
		.trim()
		.replace(/[\\/:*?"<>|\s]+/g, "_")
		.replace(/^_+|_+$/g, "") || "banco"

const downloadText = (filename: string, content: string, mime: string): void => {
	const blob = new Blob([content], { type: mime })
	const url = URL.createObjectURL(blob)
	const link = document.createElement("a")
	link.href = url
	link.download = filename
	link.click()
	URL.revokeObjectURL(url)
}

const toQuestionExport = (bank: QuestionBank): QuestionExport[] =>
	bank.questions.map((question) => ({
		prompt: question.prompt,
		options: question.options,
		answerIndex: question.answer_index ?? question.answerIndex ?? 0,
		justification: question.justification ?? undefined,
	}))

export const exportBankMarkdown = (bank: QuestionBank): void => {
	const questions = toQuestionExport(bank)
	downloadText(
		`${sanitizeFileName(bank.name)}.md`,
		bankToMarkdown(bank.name, questions),
		"text/markdown"
	)
}
