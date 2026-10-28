<script lang="ts">
	import { toast } from "svelte-sonner"
	import { Save, X } from "lucide-svelte"
	import { fade, scale } from "svelte/transition"
	import { queryClient, useMutation } from "$lib/shared/http/tanstack"

	import { quizzesService } from "../service"
	import { CertaintyValue } from "$lib/shared/value-objects/certainty.value"

	import type { Quiz } from "../entity"
	import type { CertaintyTableDTO, UpdateQuizDTO } from "../dtos"

	interface EditQuizModalProps {
		quiz: Quiz
		courseId: string
		onclose: () => void
	}

	let { quiz, courseId, onclose }: EditQuizModalProps = $props()

	const levels = ["low", "medium", "high"] as const

	const toLocalInput = (iso: string) => {
		const date = new Date(iso)
		const pad = (n: number) => String(n).padStart(2, "0")
		return (
			`${date.getFullYear()}-${pad(date.getMonth() + 1)}-${pad(date.getDate())}` +
			`T${pad(date.getHours())}:${pad(date.getMinutes())}`
		)
	}

	const currentCertainty = (): CertaintyTableDTO | null => {
		if (!quiz.certaintyTable) return null

		return {
			low: { ...quiz.certaintyTable.get("low") },
			medium: { ...quiz.certaintyTable.get("medium") },
			high: { ...quiz.certaintyTable.get("high") },
		}
	}

	// Una sola lectura al montar: el modal edita una copia, no el quiz vivo.
	/* svelte-ignore state_referenced_locally */
	let title = $state(quiz.title)
	/* svelte-ignore state_referenced_locally */
	let startsAt = $state(toLocalInput(quiz.startsAt.value))
	/* svelte-ignore state_referenced_locally */
	let duration = $state(quiz.attemptDurationMinutes)
	/* svelte-ignore state_referenced_locally */
	let questionCount = $state(quiz.questionCount)
	/* svelte-ignore state_referenced_locally */
	let multipleViews = $state(quiz.resultsMultipleViews)
	let certainty = $state(currentCertainty())

	const published = $derived(!!quiz.resultsPublishedAt)

	const changes = $derived.by((): UpdateQuizDTO => {
		const diff: UpdateQuizDTO = {}

		if (title.trim() !== quiz.title) diff.title = title.trim()
		if (multipleViews !== quiz.resultsMultipleViews) diff.resultsMultipleViews = multipleViews

		if (published) return diff

		// El input pierde segundos y milisegundos: compara contra su misma representación.
		if (startsAt !== toLocalInput(quiz.startsAt.value)) {
			const startsAtDate = new Date(startsAt)
			if (!Number.isNaN(startsAtDate.getTime())) diff.startsAt = startsAtDate.toISOString()
		}
		if (duration !== quiz.attemptDurationMinutes) diff.attemptDurationMinutes = duration
		if (questionCount !== quiz.questionCount) diff.questionCount = questionCount

		const original = quiz.certaintyTable
		if (certainty && original) {
			const same = levels.every(
				(level) =>
					certainty[level].correct === original.get(level).correct &&
					certainty[level].incorrect === original.get(level).incorrect
			)
			if (!same) diff.certaintyConfig = certainty
		}

		return diff
	})

	const problem = $derived.by(() => {
		if (!title.trim()) return "El título es obligatorio."
		if (title.trim().length > 100) return "El título no puede superar los 100 caracteres."
		if (published) return null
		if (Number.isNaN(new Date(startsAt).getTime())) return "La fecha de inicio es obligatoria."
		if (changes.startsAt && new Date(changes.startsAt) <= new Date())
			return "La fecha de inicio debe estar en el futuro."
		if (!Number.isInteger(duration) || duration < 1 || duration > 240)
			return "La duración debe estar entre 1 y 240 minutos."
		if (!Number.isInteger(questionCount) || questionCount < 1 || questionCount > 100)
			return "La cantidad de preguntas debe estar entre 1 y 100."
		return null
	})

	const hasChanges = $derived(Object.keys(changes).length > 0)

	const mutation = useMutation(() => ({
		mutationFn: () => quizzesService.update(quiz.id, changes),
		onSuccess: async (updated) => {
			toast.success(`Cuestionario ${updated.title} actualizado.`)
			await queryClient.invalidateQueries({ queryKey: ["quizzes", "course", courseId] })
			onclose()
		},
		onError: (error) => {
			toast.error(error.messageOrDefault)
		},
	}))

	const submit = (event: SubmitEvent) => {
		event.preventDefault()
		if (mutation.isPending || problem || !hasChanges) return
		mutation.mutate()
	}
</script>

<div
	class="fixed inset-0 z-50 flex items-center justify-center bg-(--scrim) p-4"
	role="dialog"
	aria-modal="true"
	tabindex="-1"
	in:fade={{ duration: 180 }}
	onclick={onclose}
	onkeydown={(e) => {
		if (e.key === "Escape" && !mutation.isPending) onclose()
	}}
>
	<section
		class="panel-elevated max-h-full w-full max-w-xl overflow-y-auto p-5"
		role="presentation"
		tabindex="-1"
		in:scale={{ duration: 190, start: 0.98 }}
		onclick={(e) => e.stopPropagation()}
	>
		<div class="mb-3 flex items-center justify-between gap-2">
			<h4 class="m-0 text-base text-black">Editar cuestionario</h4>
			<button class="btn-close" type="button" onclick={onclose} aria-label="Cerrar">
				<X size={18} aria-hidden="true" />
			</button>
		</div>

		<form class="form-stack" onsubmit={submit}>
			<label class="grid gap-1.5">
				<span class="text-sm text-zinc-800">Título</span>
				<input class="input-base" bind:value={title} maxlength="100" />
			</label>

			<div class="grid gap-3 sm:grid-cols-2">
				<label class="grid gap-1.5">
					<span class="text-sm text-zinc-800">Inicio</span>
					<input
						class="input-base h-10"
						type="datetime-local"
						bind:value={startsAt}
						disabled={published}
					/>
				</label>
				<label class="grid gap-1.5">
					<span class="text-sm text-zinc-800">Duración (min)</span>
					<input
						class="input-base"
						type="number"
						min="1"
						max="240"
						bind:value={duration}
						disabled={published}
					/>
				</label>
			</div>

			<label class="grid gap-1.5">
				<span class="text-sm text-zinc-800">Cantidad de preguntas</span>
				<input
					class="input-base"
					type="number"
					min="1"
					max="100"
					bind:value={questionCount}
					disabled={published}
				/>
			</label>

			{#if certainty}
				<div class="grid gap-2">
					<span class="text-sm text-zinc-800">Tabla de certeza</span>
					<div class="overflow-x-auto rounded-[4px] border border-zinc-200">
						<table class="w-full border-collapse text-xs">
							<thead class="text-zinc-500">
								<tr class="border-b border-zinc-200">
									<th class="px-3 py-2 text-left font-medium">Nivel</th>
									<th class="px-3 py-2 text-left font-medium">Correcta</th>
									<th class="px-3 py-2 text-left font-medium">Incorrecta</th>
								</tr>
							</thead>
							<tbody>
								{#each levels as level (level)}
									<tr class="border-b border-zinc-200/70 last:border-b-0">
										<td class="px-3 py-2 text-zinc-700">
											{CertaintyValue.format(level)}
										</td>
										<td class="px-2 py-1.5">
											<input
												type="number"
												class="input-base h-10"
												bind:value={certainty[level].correct}
												disabled={published}
											/>
										</td>
										<td class="px-2 py-1.5">
											<input
												type="number"
												class="input-base h-10"
												bind:value={certainty[level].incorrect}
												disabled={published}
											/>
										</td>
									</tr>
								{/each}
							</tbody>
						</table>
					</div>
				</div>
			{/if}

			<label class="flex cursor-pointer items-start gap-2.5">
				<input type="checkbox" class="mt-0.5" bind:checked={multipleViews} />
				<span class="grid gap-0.5">
					<span class="text-sm text-zinc-800">
						Permitir revisar los resultados más de una vez
					</span>
					<span class="text-xs text-zinc-500">
						Si no la marcas, cada estudiante ve sus resultados una sola vez.
					</span>
				</span>
			</label>

			<p class="m-0 text-xs text-zinc-500">
				{#if published}
					Los resultados ya están publicados: solo puedes cambiar el título y la revisión
					de resultados.
				{:else}
					Si ya hay intentos, solo puedes cambiar el título y la revisión de resultados.
				{/if}
			</p>

			{#if problem}
				<p class="m-0 text-sm text-red-700" role="status">{problem}</p>
			{/if}

			<div class="flex justify-end gap-2">
				<button
					class="btn-tertiary"
					type="button"
					onclick={onclose}
					disabled={mutation.isPending}
				>
					Cancelar
				</button>
				<button
					class="btn-primary flex items-center gap-1.5"
					type="submit"
					disabled={mutation.isPending || !!problem || !hasChanges}
				>
					<Save size={16} aria-hidden="true" />
					{mutation.isPending ? "Guardando..." : "Guardar cambios"}
				</button>
			</div>
		</form>
	</section>
</div>
