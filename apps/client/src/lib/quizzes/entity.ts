import type { CourseDTO } from "$lib/courses/dtos"
import type { CertaintyScoreDTO, CertaintyTableDTO, QuizDTO } from "./dtos"

import { DateValue } from "$lib/shared/value-objects/date.value"
import { QuizKindValue } from "$lib/shared/value-objects/quiz-kind.values"

export class Quiz {
	constructor(
		public id: string,
		public title: string,
		public kind: QuizKindValue,
		public joinCode: string,
		public questionCount: number,
		public certaintyTable: CertaintyTable | null,
		public attemptDurationMinutes: number,
		public startsAt: DateValue,
		public resultsPublishedAt: DateValue | null,
		public createdAt: DateValue,
		public course: Omit<CourseDTO, "members">,
		public resultsMultipleViews: boolean
	) {}

	static fromDTO(dto: QuizDTO): Quiz {
		let certaintyTable = null

		if (dto.certaintyTable) {
			certaintyTable = CertaintyTable.fromDTO(dto.certaintyTable)
		}

		const startsAt = DateValue.from(dto.startsAt)!
		const createdAt = DateValue.from(dto.createdAt)!

		const resultsPublishedAt = dto.resultsPublishedAt
			? DateValue.from(dto.resultsPublishedAt)
			: null

		const quizKind = QuizKindValue.from(dto.kind)

		return new Quiz(
			dto.id,
			dto.title,
			quizKind,
			dto.joinCode,
			dto.questionCount,
			certaintyTable,
			dto.attemptDurationMinutes,
			startsAt,
			resultsPublishedAt,
			createdAt,
			dto.course,
			dto.resultsMultipleViews
		)
	}

	public isCertaintyBased(): boolean {
		return this.kind.isCertainty()
	}

	public isTraditional(): boolean {
		return this.kind.isTraditional()
	}
}

export class CertaintyScore {
	constructor(
		public correct: number,
		public incorrect: number
	) {}

	static fromDTO(dto: CertaintyScoreDTO): CertaintyScore {
		return new CertaintyScore(dto.correct, dto.incorrect)
	}
}

export class CertaintyTable {
	private readonly values: {
		low: CertaintyScore
		medium: CertaintyScore
		high: CertaintyScore
	}

	constructor(low: CertaintyScore, medium: CertaintyScore, high: CertaintyScore) {
		this.values = {
			low,
			medium,
			high,
		}
	}

	static fromDTO(dto: CertaintyTableDTO): CertaintyTable {
		const low = CertaintyScore.fromDTO(dto.low)
		const medium = CertaintyScore.fromDTO(dto.medium)
		const high = CertaintyScore.fromDTO(dto.high)

		return new CertaintyTable(low, medium, high)
	}

	public get(certaintyLevel: "low" | "medium" | "high"): CertaintyScore {
		return this.values[certaintyLevel]
	}
}
