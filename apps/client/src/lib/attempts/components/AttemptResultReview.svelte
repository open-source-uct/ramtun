<script lang="ts">
	import type { AttemptResult, CertaintyLevel } from "$lib/attempts/dtos"

	import { DateValue } from "$lib/shared/value-objects/date.value"
	import { GradeValue } from "$lib/shared/value-objects/grade.value"
	import Markdown from "$lib/shared/components/Markdown.svelte"
	import { User, ChartColumn, Check, X, Minus, Trophy } from "lucide-svelte"

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

	const resultOf = (question: AttemptResult["questions"][number]) => {
		if (question.answerIndex === null) return "none"
		return question.isCorrect ? "ok" : "bad"
	}

	const resultStyles = {
		ok: "bg-(--ok)/8 text-(--ok)",
		bad: "bg-(--danger)/8 text-(--danger)",
		none: "bg-amber-700/10 text-amber-800",
	}

	const resultLabels = { ok: "Correcta", bad: "Incorrecta", none: "Sin responder" }
</script>

<section class="flex h-full min-h-0 flex-col gap-4">
	<div class="grid grid-cols-2 gap-3 sm:grid-cols-[1.5fr_1fr_1fr_1fr]">
		<div class="panel-surface col-span-2 p-3 sm:col-span-1 sm:p-4">
			<p class="m-0 flex items-center gap-2 text-xs font-medium text-zinc-700">
				<User size={13} /> Estudiante
			</p>
			<p class="mt-1 mb-0 text-lg font-semibold text-black">
				{result.userName}
			</p>
		</div>
		<div class="panel-surface p-3 sm:p-4">
			<p class="m-0 flex items-center gap-2 text-xs font-medium text-zinc-700">
				<Trophy size={13} /> Nota
			</p>
			<p class="mt-1 mb-0 text-lg font-semibold text-black">
				{GradeValue.format(result.grade)}
			</p>
		</div>
		<div class="panel-surface p-3 sm:p-4">
			<p class="m-0 flex items-center gap-2 text-xs font-medium text-zinc-700">
				<ChartColumn size={13} /> Puntos
			</p>
			<p class="mt-1 mb-0 text-lg font-semibold text-black">
				{result.score} / {result.maxScore}
			</p>
		</div>
		<div class="panel-surface col-span-2 p-3 sm:col-span-1 sm:p-4">
			<p class="m-0 flex items-center gap-2 text-xs font-medium text-zinc-700">Enviado</p>
			<p class="mt-1 mb-0 text-lg font-semibold text-black">
				{DateValue.format(result.submittedAt)}
			</p>
		</div>
	</div>

	<div class="min-h-0 space-y-4 overflow-auto pr-1">
		{#each result.questions as question, index (question.questionId)}
			{@const outcome = resultOf(question)}
			<article class="panel-surface p-4 sm:p-5">
				<div
					class="flex flex-col gap-2 sm:flex-row sm:items-start sm:justify-between sm:gap-3"
				>
					<div class="flex min-w-0 flex-1 items-start gap-2">
						<span class="shrink-0 font-medium text-black">{index + 1}.</span>
						<div class="min-w-0 flex-1">
							<Markdown content={question.question} />
						</div>
					</div>
					<span
						class={`inline-flex h-6.5 shrink-0 items-center gap-1.5 self-start rounded-full px-2.5 text-[13px] font-semibold whitespace-nowrap ${resultStyles[outcome]}`}
					>
						{#if outcome === "ok"}
							<Check size={14} />
						{:else if outcome === "bad"}
							<X size={14} />
						{:else}
							<Minus size={14} />
						{/if}
						{resultLabels[outcome]}
						{#if outcome !== "none"}
							<span class="opacity-50">·</span>
							<span class="tabular-nums"
								>{formatSignedPoints(question.awardedPoints)} pt</span
							>
						{/if}
					</span>
				</div>

				<div class="mt-4 grid gap-1.5">
					{#each question.options as option, optionIndex (`${question.questionId}-${optionIndex}`)}
						{@const isCorrect = optionIndex === question.correctAnswerIndex}
						{@const isSelected = optionIndex === question.answerIndex}
						{@const isWrong = isSelected && !isCorrect}
						<div
							class={`flex flex-wrap items-center gap-x-3 gap-y-1 rounded-sm border px-3 py-2.5 sm:flex-nowrap ${
								isCorrect
									? "border-transparent bg-(--ok)/8"
									: isWrong
										? "border-transparent bg-(--danger)/8"
										: "border-zinc-200"
							}`}
						>
							<span
								class={`grid size-5 shrink-0 place-items-center rounded-full border-[1.5px] ${
									isCorrect && isSelected
										? "border-(--ok) bg-(--ok) text-white"
										: isCorrect
											? "border-(--ok) text-(--ok)"
											: isWrong
												? "border-(--danger) bg-(--danger) text-white"
												: "border-zinc-300"
								}`}
							>
								{#if isCorrect}
									<Check size={12} strokeWidth={3} />
								{:else if isWrong}
									<X size={12} strokeWidth={3} />
								{/if}
							</span>
							<div class="min-w-0 flex-1 text-base leading-relaxed text-black">
								<Markdown content={option} inline />
							</div>
							{#if isSelected || isCorrect}
								<span
									class={`basis-full pl-8 text-xs font-semibold sm:basis-auto sm:pl-0 ${
										isWrong
											? "text-(--danger)"
											: isCorrect
												? "text-(--ok)"
												: "text-zinc-600"
									}`}
								>
									{isSelected ? "Tu respuesta" : "Respuesta correcta"}
								</span>
							{/if}
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
					{@const filled = certaintyLevels.findIndex(
						(item) => item.level === question.certaintyLevel
					)}
					<div
						class="mt-4 flex items-center gap-2.5 border-t border-zinc-200 pt-4 text-sm text-zinc-600"
					>
						Nivel de certeza marcado
						<span class="flex gap-0.5" aria-hidden="true">
							{#each certaintyLevels as item, i (item.level)}
								<span
									class={`h-1.5 w-4 rounded-full ${i <= filled ? "bg-zinc-800" : "bg-zinc-200"}`}
								></span>
							{/each}
						</span>
						<span class="font-medium text-black">{certaintyLevels[filled]?.label}</span>
					</div>
				{/if}
			</article>
		{/each}
	</div>
</section>
