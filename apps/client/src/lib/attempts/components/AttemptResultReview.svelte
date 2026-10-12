<script lang="ts">
	import type { AttemptResult, CertaintyLevel } from "$lib/attempts/dtos"

	import { DateValue } from "$lib/shared/value-objects/date.value"
	import { GradeValue } from "$lib/shared/value-objects/grade.value"
	import Markdown from "$lib/shared/components/Markdown.svelte"
	import { User, ChartColumn, CircleCheck, CircleX, Trophy } from "lucide-svelte"

	let { result }: { result: AttemptResult } = $props()

	const formatPoints = (value: number) =>
		new Intl.NumberFormat("es-CL", {
			minimumFractionDigits: 0,
			maximumFractionDigits: 2,
		}).format(value)

	const formatSignedPoints = (value: number) => {
		const formatted = formatPoints(value)
		if (value > 0) return `+${formatted}`
		if (value < 0) return `-${formatPoints(Math.abs(value))}`
		return formatted
	}

	const certaintyLevels: { level: CertaintyLevel; label: string }[] = [
		{ level: "low", label: "Baja" },
		{ level: "medium", label: "Media" },
		{ level: "high", label: "Alta" },
	]

	const getOptionClass = (
		optionIndex: number,
		correctAnswerIndex: number,
		answerIndex: number | null
	) => {
		if (optionIndex === correctAnswerIndex) {
			return "border-emerald-700 bg-emerald-50 text-emerald-900"
		}

		if (
			answerIndex !== null &&
			optionIndex === answerIndex &&
			answerIndex !== correctAnswerIndex
		) {
			return "border-red-700 bg-red-50 text-red-900"
		}

		if (answerIndex !== null && optionIndex === answerIndex) {
			return "border-black bg-black text-white"
		}

		return "border-zinc-300 bg-white text-black"
	}
</script>

<section class="flex h-full min-h-0 flex-col gap-4">
	<div class="grid gap-3 sm:grid-cols-[1.5fr_1fr_1fr_1fr]">
		<div class="panel-muted p-3 sm:p-4">
			<p class="m-0 flex items-center gap-2 text-xs font-medium text-zinc-700">
				<User size={13} /> Estudiante
			</p>
			<p class="mt-1 mb-0 text-lg font-semibold text-black">
				{result.userName}
			</p>
		</div>
		<div class="panel-muted p-3 sm:p-4">
			<p class="m-0 flex items-center gap-2 text-xs font-medium text-zinc-700">
				<Trophy size={13} /> Nota
			</p>
			<p class="mt-1 mb-0 text-lg font-semibold text-black">
				{GradeValue.format(result.grade)}
			</p>
		</div>
		<div class="panel-muted p-3 sm:p-4">
			<p class="m-0 flex items-center gap-2 text-xs font-medium text-zinc-700">
				<ChartColumn size={13} /> Puntos
			</p>
			<p class="mt-1 mb-0 text-lg font-semibold text-black">
				{result.score} / {result.maxScore}
			</p>
		</div>
		<div class="panel-muted p-3 sm:p-4">
			<p class="m-0 flex items-center gap-2 text-xs font-medium text-zinc-700">Enviado</p>
			<p class="mt-1 mb-0 text-lg font-semibold text-black">
				{DateValue.format(result.submittedAt)}
			</p>
		</div>
	</div>

	<div class="min-h-0 space-y-4 overflow-auto pr-1">
		{#each result.questions as question, index (question.questionId)}
			<article class="panel-muted p-4 sm:p-5">
				<div class="flex items-start justify-between gap-3">
					<div class="min-w-0 flex-1">
						<div class="flex items-start gap-2">
							<span class="shrink-0 font-medium text-black">{index + 1}.</span>
							<div class="min-w-0 flex-1">
								<Markdown content={question.question} />
							</div>
						</div>
					</div>
					{#if question.isCorrect}
						<span
							class="inline-flex shrink-0 items-center gap-1 text-sm font-medium text-emerald-700"
						>
							<CircleCheck size={14} /> Correcta
						</span>
					{:else}
						<span
							class="inline-flex shrink-0 items-center gap-1 text-sm font-medium text-red-700"
						>
							<CircleX size={14} /> Incorrecta
						</span>
					{/if}
				</div>

				<div class="mt-4 grid gap-2.5">
					{#each question.options as option, optionIndex (`${question.questionId}-${optionIndex}`)}
						{@const isSelected =
							question.answerIndex !== null && optionIndex === question.answerIndex}
						<div
							class={`rounded-sm border px-4 py-3 text-left text-base leading-relaxed ${getOptionClass(
								optionIndex,
								question.correctAnswerIndex,
								question.answerIndex
							)}`}
						>
							<div
								class="flex flex-col gap-2 sm:flex-row sm:items-center sm:justify-between sm:gap-3"
							>
								<div class="min-w-0 flex-1">
									<Markdown content={option} inline />
								</div>
								{#if isSelected}
									<div class="flex flex-wrap gap-2 sm:shrink-0">
										<span
											class="inline-flex h-6 min-w-12 shrink-0 items-center justify-center rounded-sm border border-zinc-300 bg-white/90 px-2 text-xs font-semibold text-zinc-700"
										>
											{formatSignedPoints(question.awardedPoints)}pt
										</span>
										<span
											class="inline-flex h-6 min-w-16 shrink-0 items-center justify-center rounded-sm border border-blue-500 bg-blue-50 px-2 text-xs font-semibold text-blue-800"
										>
											Alternativa seleccionada
										</span>
									</div>
								{/if}
							</div>
						</div>
					{/each}
				</div>

				{#if question.justification}
					<div
						class="mt-4 grid grid-cols-[minmax(0,1fr)] gap-2 border-t border-zinc-200 pt-4"
					>
						<p class="m-0 text-sm font-medium text-black">Justificación</p>
						<Markdown content={question.justification} />
					</div>
				{/if}

				{#if question.certaintyLevel}
					<div class="mt-4 grid gap-2 border-t border-zinc-200 pt-4">
						<p class="m-0 text-sm font-medium text-black">Nivel de certeza marcado</p>
						<div class="grid gap-2 sm:grid-cols-3">
							{#each certaintyLevels as item (item.level)}
								<div
									class={`rounded-sm border px-4 py-3 text-left transition ${
										question.certaintyLevel === item.level
											? "border-black bg-black text-white shadow-[0_10px_20px_rgba(0,0,0,0.08)]"
											: "border-zinc-300 bg-white text-black"
									}`}
								>
									<span class="block text-sm font-medium">
										{item.label}
									</span>
								</div>
							{/each}
						</div>
					</div>
				{/if}

				{#if question.answerIndex === null}
					<p class="mt-4 mb-0 text-sm font-bold text-amber-700">
						Sin respuesta en esta pregunta.
					</p>
				{/if}
			</article>
		{/each}
	</div>
</section>
