<script lang="ts">
	import * as v from "valibot"
	import type { CreateMutationResult } from "@tanstack/svelte-query"
	import type { CreateQuestionBankInput } from "$lib/banks/banks.dtos"
	import type { ApiResponse } from "$lib/shared/http/response"
	import type { SubmitEventHandler } from "@formisch/svelte"
	import type { QuestionInput } from "$lib/banks/banks.dtos"
	import type { ParsedBank } from "$lib/banks/markdown"

	import { scale } from "svelte/transition"
	import { toast } from "svelte-sonner"
	import { Upload, Code, X } from "lucide-svelte"
	import { bankQuestionsSchema } from "$lib/banks/banks.dtos"
	import { createForm, Field, Form, getInput, reset, setInput } from "@formisch/svelte"
	import { inlineTryAsync } from "$lib/shared/try"
	import { fileNameWithoutExtension, parseBankMarkdown } from "$lib/banks/markdown"

	import CodeBlock from "$lib/shared/components/CodeBlock.svelte"

	interface BankUploadModalProps {
		open: boolean
		courseId: string
		onclose: () => void
		onsuccess: () => void
		mutation: CreateMutationResult<void, ApiResponse<unknown>, CreateQuestionBankInput, unknown>
	}

	let { open, courseId, onclose, onsuccess, mutation }: BankUploadModalProps = $props()

	const bankUploadFormSchema = v.object({
		name: v.pipe(
			v.string(),
			v.trim(),
			v.minLength(1, "El nombre es obligatorio."),
			v.maxLength(120, "Máximo 120 caracteres.")
		),
	})

	const form = createForm({
		schema: bankUploadFormSchema,
		initialInput: { name: "" },
	})

	let selectedFile = $state<File | null>(null)
	let parsedQuestions = $state<QuestionInput[]>([])
	let fileError = $state<string | null>(null)
	let fileInput = $state<HTMLInputElement | null>(null)
	let showFormatModal = $state(false)

	const mdExample = `# Guía 1 - Mecánica

### ¿Cuál es la capital de Francia?
Pregunta con un bloque de código:

\`\`\`python
print("hola")
\`\`\`

- [X] París
- [ ] Londres
- [ ] Berlín

La capital de Francia es París desde hace siglos.

### ¿Cuánto es 2 + 2?
- [X] 4
- [ ] 3
- [ ] 5`

	const handleFileChange = async (e: Event) => {
		const file = (e.target as HTMLInputElement).files?.[0] ?? null
		const replacesFile = selectedFile !== null
		selectedFile = file
		fileError = null
		parsedQuestions = []

		if (!file) return

		const [parsed, parseError] = await inlineTryAsync<ParsedBank>(async () => {
			const text = await file.text()
			return parseBankMarkdown(text, fileNameWithoutExtension(file.name))
		})

		if (parseError !== null || !parsed) {
			fileError = parseError?.message ?? "El archivo no es un Markdown válido."
			return
		}

		const result = v.safeParse(bankQuestionsSchema, parsed.questions)
		if (!result.success) {
			fileError = result.issues[0]?.message ?? "El archivo Markdown no es válido."
			return
		}

		parsedQuestions = result.output
		// El archivo propone el nombre; un nombre escrito antes del primer archivo se respeta.
		if (replacesFile || !getInput(form, { path: ["name"] })) {
			setInput(form, { path: ["name"], input: parsed.name })
		}
	}

	const handleSubmit: SubmitEventHandler<typeof bankUploadFormSchema> = async (output) => {
		if (!selectedFile) {
			toast.error("Selecciona un archivo Markdown.")
			return
		}

		if (fileError) {
			toast.error("Corrige los errores del archivo antes de continuar.")
			return
		}

		await mutation.mutateAsync({
			courseId,
			name: output.name,
			questions: parsedQuestions,
		})

		reset(form)
		selectedFile = null
		parsedQuestions = []
		fileError = null
		if (fileInput) fileInput.value = ""

		onsuccess()
	}
</script>

{#if open}
	<div
		class="fixed inset-0 z-50 flex items-center justify-center bg-black/40 p-4"
		role="dialog"
		aria-modal="true"
		tabindex="-1"
		onclick={onclose}
		onkeydown={(e) => {
			if (e.key === "Escape") onclose()
		}}
	>
		<section
			class="panel-elevated w-full max-w-2xl p-5"
			role="presentation"
			tabindex="-1"
			transition:scale={{ duration: 190, start: 0.98 }}
			onclick={(e) => e.stopPropagation()}
		>
			<div class="mb-3 flex items-center justify-between gap-2">
				<h4 class="m-0 text-base text-black">Subir banco de preguntas</h4>
				<button class="btn-close" type="button" onclick={onclose} aria-label="Cerrar">
					<X size={18} aria-hidden="true" />
				</button>
			</div>
			<section class="page-main">
				<div class="form-stack">
					<div class="form-stack mt-3">
						<Form of={form} onsubmit={handleSubmit} class="form-stack">
							<Field of={form} path={["name"]}>
								{#snippet children(field)}
									<label class="grid gap-1.5">
										<span class="text-sm text-zinc-800">Nombre del banco</span>
										<input
											{...field.props}
											class="input-base"
											value={field.input ?? ""}
											placeholder="Ej: Guía 1 - Mecánica"
										/>
										{#if field.errors?.[0]}
											<span class="text-sm text-red-700"
												>{field.errors[0]}</span
											>
										{/if}
									</label>
								{/snippet}
							</Field>

							<div class="grid gap-1.5">
								<span class="text-sm text-zinc-800">Archivo Markdown</span>
								<label
									class="btn-secondary file-label flex cursor-pointer items-center justify-start gap-1.5 text-left"
								>
									<Upload size={16} aria-hidden="true" />
									{selectedFile
										? selectedFile.name
										: "Seleccionar archivo Markdown"}
									<input
										type="file"
										accept=".md,.markdown,text/markdown"
										class="sr-only"
										onchange={handleFileChange}
										bind:this={fileInput}
									/>
								</label>
								{#if fileError}
									<span class="text-sm text-red-700">{fileError}</span>
								{/if}
							</div>

							<div class="flex gap-2">
								<button
									class="btn-primary flex-1 gap-1.5"
									type="submit"
									disabled={mutation.isPending || !selectedFile || !!fileError}
								>
									<Upload size={16} aria-hidden="true" />
									{mutation.isPending ? "Subiendo..." : "Subir banco"}
								</button>
								<button
									class="btn-secondary flex-1 gap-1.5"
									type="button"
									onclick={() => (showFormatModal = true)}
								>
									<Code size={16} aria-hidden="true" />
									Ver formato
								</button>
							</div>
						</Form>
					</div>
				</div>
			</section>
		</section>
	</div>
{/if}

{#if showFormatModal}
	<div
		class="fixed inset-0 z-60 flex items-center justify-center bg-black/40 p-4"
		role="dialog"
		aria-modal="true"
		tabindex="-1"
		onclick={() => (showFormatModal = false)}
		onkeydown={(e) => {
			if (e.key === "Escape") showFormatModal = false
		}}
	>
		<div
			class="w-full max-w-2xl rounded-lg bg-white p-6"
			role="presentation"
			tabindex="-1"
			onclick={(e) => e.stopPropagation()}
		>
			<h4 class="m-0 text-lg text-black">Formato Markdown esperado</h4>
			<p class="mt-2 mb-4 text-sm text-zinc-600">
				El archivo debe ser un documento Markdown con una pregunta por sección.
			</p>

			<p class="mt-2 mb-1 text-sm font-semibold text-zinc-800">Formato</p>
			<CodeBlock code={mdExample} class="mt-2" />

			<ul class="mt-4 mb-0 list-inside list-disc text-sm text-zinc-700">
				<li>
					<code class="text-zinc-900"># Nombre</code> (primera línea, opcional): nombre del
					banco; si falta, se usa el nombre del archivo
				</li>
				<li>
					<code class="text-zinc-900">### Título</code>: separa cada pregunta; el texto
					entre el título y las opciones es el enunciado (soporta Markdown y LaTeX)
				</li>
				<li>
					<code class="text-zinc-900">- [X]</code> marca la opción correcta y
					<code class="text-zinc-900">- [ ]</code> las demás. Debe haber exactamente una correcta
					y entre 2 y 5 opciones
				</li>
				<li>
					Texto después de la última opción: justificación opcional que el estudiante
					ve al revisar sus resultados (soporta Markdown y LaTeX)
				</li>
			</ul>

			<div class="mt-5 flex justify-end">
				<button class="btn-primary" type="button" onclick={() => (showFormatModal = false)}>
					Cerrar
				</button>
			</div>
		</div>
	</div>
{/if}
