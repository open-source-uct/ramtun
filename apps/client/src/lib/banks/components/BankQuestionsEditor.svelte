<script lang="ts" module>
	export type DraftQuestion = {
		key: string
		prompt: string
		options: string[]
		answerIndex: number
		justification: string
	}

	export const emptyDraftQuestion = (): DraftQuestion => ({
		key: crypto.randomUUID(),
		prompt: "",
		options: ["", ""],
		answerIndex: 0,
		justification: "",
	})
</script>

<script lang="ts">
	import { Plus, Trash2 } from "lucide-svelte"
	import { SvelteSet } from "svelte/reactivity"
	import {
		MAX_OPTIONS_PER_QUESTION,
		MAX_QUESTIONS_PER_BANK,
		MIN_OPTIONS_PER_QUESTION,
	} from "$lib/banks/banks.dtos"
	import Markdown from "$lib/shared/components/Markdown.svelte"

	interface BankQuestionsEditorProps {
		questions: DraftQuestion[]
		disabled?: boolean
	}

	let { questions = $bindable(), disabled = false }: BankQuestionsEditorProps = $props()

	// Claves de las preguntas que se muestran en vista previa; el resto está en modo escritura.
	const previewing = new SvelteSet<string>()

	const setPreview = (key: string, preview: boolean) => {
		if (preview) previewing.add(key)
		else previewing.delete(key)
	}

	const addQuestion = () => {
		questions.push(emptyDraftQuestion())
	}

	const removeQuestion = (index: number) => {
		questions.splice(index, 1)
	}

	const addOption = (question: DraftQuestion) => {
		question.options.push("")
	}

	// Mantiene `answerIndex` apuntando a la misma opción tras quitar una anterior.
	const removeOption = (question: DraftQuestion, optionIndex: number) => {
		question.options.splice(optionIndex, 1)

		if (optionIndex < question.answerIndex) question.answerIndex -= 1
		else if (optionIndex === question.answerIndex) question.answerIndex = 0
	}
</script>

<div class="grid gap-4">
	{#each questions as question, index (question.key)}
		{@const preview = previewing.has(question.key)}
		<article class="panel-surface p-4 sm:p-5">
			<div class="mb-6 flex flex-wrap items-center justify-between gap-3">
				<h4 class="m-0 text-[17px] font-medium text-(--ink)">Pregunta {index + 1}</h4>
				<div class="flex items-center gap-2">
					<div
						class="segmented segmented-sm"
						role="group"
						aria-label={`Modo de la pregunta ${index + 1}`}
					>
						<button
							type="button"
							class="segmented-item"
							data-active={!preview}
							onclick={() => setPreview(question.key, false)}
						>
							Escribir
						</button>
						<button
							type="button"
							class="segmented-item"
							data-active={preview}
							onclick={() => setPreview(question.key, true)}
						>
							Vista previa
						</button>
					</div>
					<button
						class="icon-btn icon-btn-danger"
						type="button"
						aria-label={`Eliminar pregunta ${index + 1}`}
						title="Eliminar pregunta"
						onclick={() => removeQuestion(index)}
						disabled={disabled || questions.length <= 1}
					>
						<Trash2 size={16} aria-hidden="true" />
					</button>
				</div>
			</div>

			{#if preview}
				<div class="grid gap-8">
					<section class="grid gap-2.5">
						<h5 class="section-title">Enunciado</h5>
						{#if question.prompt.trim()}
							<Markdown content={question.prompt} />
						{:else}
							<p class="empty-preview">Sin contenido.</p>
						{/if}
					</section>

					<section class="grid gap-2.5">
						<h5 class="section-title">Opciones</h5>
						<ul class="m-0 grid list-none gap-2 p-0">
							{#each question.options as option, optionIndex (optionIndex)}
								{@const correct = question.answerIndex === optionIndex}
								<li
									class="flex items-center gap-2.5 rounded-[4px] border px-3 py-2 text-[15px]"
									class:border-(--line)={!correct}
									class:border-emerald-700={correct}
									class:bg-emerald-50={correct}
								>
									<span class="min-w-0 flex-1">
										{#if option.trim()}
											<Markdown content={option} inline />
										{:else}
											<span class="empty-preview"
												>Opción {optionIndex + 1} vacía</span
											>
										{/if}
									</span>
									{#if correct}
										<span class="text-xs text-emerald-700">Correcta</span>
									{/if}
								</li>
							{/each}
						</ul>
					</section>

					{#if question.justification.trim()}
						<section class="grid gap-2.5">
							<h5 class="section-title">Justificación</h5>
							<Markdown content={question.justification} />
						</section>
					{/if}
				</div>
			{:else}
				<div class="grid gap-8">
					<label class="grid gap-2.5">
						<span class="section-title">
							Enunciado <span class="section-hint">Markdown y LaTeX</span>
						</span>
						<textarea
							class="input-base field-sizing-content min-h-16 resize-none font-mono"
							bind:value={question.prompt}
							placeholder="Escribe el enunciado en Markdown..."
						></textarea>
					</label>

					<fieldset class="m-0 grid gap-2.5 border-0 p-0">
						<div class="flex min-h-8 items-center justify-between gap-3">
							<legend class="section-title float-left p-0">
								Opciones <span class="section-hint"
									>Marca la respuesta correcta</span
								>
							</legend>
							<button
								class="btn-tertiary btn-sm shrink-0"
								type="button"
								onclick={() => addOption(question)}
								disabled={disabled ||
									question.options.length >= MAX_OPTIONS_PER_QUESTION}
							>
								<Plus size={14} aria-hidden="true" />
								Agregar opción
							</button>
						</div>
						{#each question.options as _, optionIndex (optionIndex)}
							<div class="flex items-center gap-2">
								<input
									class="[--check:var(--ok)]"
									type="radio"
									name={`correct-${question.key}`}
									checked={question.answerIndex === optionIndex}
									onchange={() => (question.answerIndex = optionIndex)}
									aria-label={`Opción ${optionIndex + 1} es la correcta`}
								/>
								<input
									class="input-base min-w-0 flex-1"
									bind:value={question.options[optionIndex]}
									placeholder={`Opción ${optionIndex + 1}`}
									aria-label={`Texto de la opción ${optionIndex + 1}`}
								/>
								<button
									class="icon-btn shrink-0"
									type="button"
									aria-label={`Quitar opción ${optionIndex + 1}`}
									title="Quitar opción"
									onclick={() => removeOption(question, optionIndex)}
									disabled={disabled ||
										question.options.length <= MIN_OPTIONS_PER_QUESTION}
								>
									<Trash2 size={16} aria-hidden="true" />
								</button>
							</div>
						{/each}
					</fieldset>

					<label class="grid gap-2.5">
						<span class="section-title">
							Justificación <span class="section-hint">Opcional</span>
						</span>
						<textarea
							class="input-base field-sizing-content min-h-16 resize-none font-mono"
							bind:value={question.justification}
							placeholder="Explica por qué la respuesta correcta lo es..."
						></textarea>
					</label>
				</div>
			{/if}
		</article>
	{/each}

	<div>
		<button
			class="btn-secondary flex items-center gap-1.5"
			type="button"
			onclick={addQuestion}
			disabled={disabled || questions.length >= MAX_QUESTIONS_PER_BANK}
		>
			<Plus size={16} aria-hidden="true" />
			Agregar pregunta
		</button>
	</div>
</div>

<style>
	.section-title {
		display: flex;
		flex-wrap: wrap;
		align-items: baseline;
		column-gap: 0.5rem;
		row-gap: 0.125rem;
		margin: 0;
		font-size: 0.9375rem;
		font-weight: 600;
		color: var(--ink);
	}

	.section-hint {
		font-size: 0.8125rem;
		font-weight: 400;
		color: var(--ink-muted);
	}

	.empty-preview {
		margin: 0;
		font-size: 0.875rem;
		color: var(--ink-muted);
	}
</style>
