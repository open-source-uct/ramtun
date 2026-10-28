import type { QuestionInput } from "$lib/banks/banks.dtos"

export type ParsedBank = {
	name: string
	questions: QuestionInput[]
}

type QuestionSection = {
	heading: string
	lines: string[]
}

const FENCE_RE = /^\s*```/
const HEADING_RE = /^#+\s+(.*)$/
const QUESTION_HEADING_RE = /^###\s+(.*)$/
const OPTION_RE = /^\s*-\s+\[([ xX])\]\s+(.+)$/
const JUSTIFICATION_START = "<!-- ramtun-justification:start -->"
const JUSTIFICATION_END = "<!-- ramtun-justification:end -->"

const fileNameWithoutExtension = (fileName: string): string => {
	const name = fileName.replace(/\.[^.]+$/, "")
	return name || "banco"
}

const questionLabel = (index: number) => `Pregunta ${index + 1}`

// El bloque explícito permite exportar justificaciones Markdown sin confundirlas con preguntas u opciones.
const parseQuestion = (section: QuestionSection, sectionIndex: number): QuestionInput => {
	const options: { checked: boolean; text: string }[] = []
	const prompt: string[] = []
	const justification: string[] = []
	let inFence = false
	let inJustification = false
	let hasJustificationText = false

	for (const line of section.lines) {
		if (FENCE_RE.test(line)) {
			inFence = !inFence
		}

		if (!inFence && line.trim() === JUSTIFICATION_START) {
			if (options.length === 0) {
				throw new Error(
					`${questionLabel(sectionIndex)}: la justificación debe ir después de las opciones.`
				)
			}

			inJustification = true
			continue
		}

		if (!inFence && line.trim() === JUSTIFICATION_END) {
			if (!inJustification) {
				throw new Error(
					`${questionLabel(sectionIndex)}: falta el inicio de la justificación.`
				)
			}

			inJustification = false
			continue
		}

		const optionMatch =
			inFence || inJustification || hasJustificationText ? null : OPTION_RE.exec(line)

		if (optionMatch) {
			options.push({
				checked: optionMatch[1].toLowerCase() === "x",
				text: optionMatch[2].trim(),
			})
			justification.length = 0
		} else if (options.length === 0) {
			prompt.push(line)
		} else {
			justification.push(line)
			hasJustificationText ||= Boolean(line.trim())
		}
	}

	if (inJustification) {
		throw new Error(`${questionLabel(sectionIndex)}: falta el cierre de la justificación.`)
	}

	let promptText = prompt.join("\n").trim()
	if (!promptText && section.heading) {
		promptText = section.heading
	}

	if (options.length < 2 || options.length > 5) {
		throw new Error(`${questionLabel(sectionIndex)}: debe tener entre 2 y 5 opciones.`)
	}

	const checkedCount = options.filter((option) => option.checked).length
	if (checkedCount !== 1) {
		throw new Error(
			`${questionLabel(sectionIndex)}: debe marcar exactamente una opción correcta con [X].`
		)
	}

	return {
		prompt: promptText,
		options: options.map((option) => option.text),
		answerIndex: options.findIndex((option) => option.checked),
		justification: justification.join("\n").trim() || undefined,
	}
}

export const parseBankMarkdown = (text: string, fallbackName: string): ParsedBank => {
	const lines = text.split(/\r?\n/)

	let name = fallbackName
	const sections: QuestionSection[] = []
	let current: QuestionSection | null = null
	let inFence = false
	let inJustification = false
	let sawHeading = false

	for (const line of lines) {
		if (FENCE_RE.test(line)) {
			inFence = !inFence
		}

		if (!inFence && line.trim() === JUSTIFICATION_START) {
			inJustification = true
			if (current) current.lines.push(line)
			continue
		}

		if (!inFence && line.trim() === JUSTIFICATION_END) {
			inJustification = false
			if (current) current.lines.push(line)
			continue
		}

		const headingMatch = !inFence && !inJustification ? HEADING_RE.exec(line) : null

		if (headingMatch) {
			const level = (line.match(/^#+/) as RegExpMatchArray)[0].length

			if (!sawHeading && level === 1) {
				name = headingMatch[1].trim()
				sawHeading = true
				current = null
				continue
			}

			const questionHeading = QUESTION_HEADING_RE.exec(line)
			if (questionHeading) {
				sawHeading = true
				current = { heading: questionHeading[1].trim(), lines: [] }
				sections.push(current)
				continue
			}
		}

		if (current) {
			current.lines.push(line)
		}
	}

	if (sections.length === 0) {
		throw new Error(
			"El archivo no contiene preguntas. Separa cada pregunta con un título (###)."
		)
	}

	if (inJustification) {
		throw new Error("Falta el cierre de la justificación.")
	}

	const questions = sections.map((section, sectionIndex) => parseQuestion(section, sectionIndex))

	return { name, questions }
}

export const bankToMarkdown = (name: string, questions: QuestionInput[]): string => {
	const blocks = questions.map((question, index) => {
		const options = question.options
			.map(
				(option, optionIndex) =>
					`- [${optionIndex === question.answerIndex ? "X" : " "}] ${option}`
			)
			.join("\n")

		const justification = question.justification
			? `\n\n${JUSTIFICATION_START}\n${question.justification}\n${JUSTIFICATION_END}`
			: ""

		return `### ${index + 1}\n${question.prompt}\n\n${options}${justification}`
	})

	return `# ${name}\n\n${blocks.join("\n\n")}\n`
}

export { fileNameWithoutExtension }
