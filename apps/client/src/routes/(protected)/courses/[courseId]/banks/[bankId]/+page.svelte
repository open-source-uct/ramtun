<script lang="ts">
	import { useQuery, useMutation, useQueryClient } from "$lib/shared/http/tanstack"
	import { FileDown, Pencil, Save, X } from "lucide-svelte"
	import { page } from "$app/state"
	import { resolve } from "$app/paths"
	import { courseSections } from "$lib/courses/sections"
	import { toast } from "svelte-sonner"
	import { banksService } from "$lib/banks/banks.service"
	import { exportBankMarkdown } from "$lib/banks/utils"
	import BreadcrumbMenu from "$lib/shared/components/BreadcrumbMenu.svelte"
	import Markdown from "$lib/shared/components/Markdown.svelte"
	import MarkdownEditor from "$lib/shared/components/MarkdownEditor.svelte"
	import Title from "$lib/shared/components/Title.svelte"

	let { data } = $props()

	const queryClient = useQueryClient()

	const bankQuery = useQuery(() => ({
		queryKey: ["bank", data.bankId],
		queryFn: () => banksService.getById(data.bankId),
	}))

	// Misma clave que la lista de bancos: comparte caché con `/banks`.
	const banksQuery = useQuery(() => ({
		queryKey: ["banks", data.courseId],
		queryFn: () => banksService.listByCourse(data.courseId),
	}))

	const sectionItems = $derived(
		courseSections(data.courseId).map(({ label, href, isActive }) => ({
			label,
			href,
			current: isActive(page.url.pathname),
		}))
	)

	const bankItems = $derived(
		(banksQuery.data ?? []).map((bank) => ({
			label: bank.name,
			href: `/courses/${data.courseId}/banks/${bank.id}`,
			current: bank.id === data.bankId,
		}))
	)

	type DraftQuestion = {
		prompt: string
		options: string[]
		justification: string
	}

	let editing = $state(false)
	let draft = $state<DraftQuestion[]>([])

	const correctIndex = (question: { answer_index?: number; answerIndex?: number }) =>
		question.answer_index ?? question.answerIndex

	const enterEdit = () => {
		const bank = bankQuery.data
		if (!bank) return

		draft = bank.questions.map((question) => ({
			prompt: question.prompt,
			options: question.options,
			justification: question.justification ?? "",
		}))
		editing = true
	}

	const cancelEdit = () => {
		editing = false
		draft = []
	}

	const updateMutation = useMutation(() => ({
		mutationFn: (questions: DraftQuestion[]) =>
			banksService.update(data.bankId, {
				questions: questions.map((question) => ({
					prompt: question.prompt,
					options: question.options,
					justification: question.justification.trim() || null,
				})),
			}),
		onSuccess: async () => {
			editing = false
			draft = []
			toast.success("Banco actualizado correctamente.")
			await queryClient.invalidateQueries({ queryKey: ["bank", data.bankId] })
			await queryClient.invalidateQueries({ queryKey: ["banks", data.courseId] })
		},
		onError: (error) => {
			toast.error(error.messageOrDefault)
		},
	}))

	const saveEdit = () => {
		if (updateMutation.isPending) return
		updateMutation.mutate(draft)
	}
</script>

<section class="grid gap-4">
	<header class="flex flex-wrap items-end justify-between gap-3">
		<div class="min-w-0">
			<nav
				class="mb-1.5 flex min-w-0 flex-wrap items-center gap-y-0.5 text-sm text-zinc-600"
				aria-label="Ubicación"
			>
				<a
					class="shrink-0 hover:text-black"
					href={resolve(`/courses/${data.courseId}/quizzes`)}
				>
					{data.course.name}
				</a>
				<BreadcrumbMenu
					label="Cambiar de sección"
					items={sectionItems}
					disabled={editing}
				/>
				<a
					class="shrink-0 hover:text-black"
					href={resolve(`/courses/${data.courseId}/banks`)}
				>
					Bancos de preguntas
				</a>
				<BreadcrumbMenu
					label="Cambiar de banco"
					items={bankItems}
					empty={banksQuery.isLoading ? "Cargando bancos..." : "Sin bancos"}
					disabled={editing}
				/>
				<span class="min-w-0 truncate text-black" aria-current="page">
					{bankQuery.data?.name ?? "Banco"}
				</span>
			</nav>
			<div class="flex min-w-0 flex-wrap items-baseline gap-x-3 gap-y-1">
				<Title size="page">{bankQuery.data?.name ?? "Banco"}</Title>
				<p class="m-0 text-sm text-zinc-700">
					{bankQuery.data?.questions.length ?? 0} preguntas
					{#if editing}
						- modo edición
					{/if}
				</p>
			</div>
		</div>
		<div class="flex flex-wrap items-center gap-1.5">
			{#if editing}
				<button
					class="btn-secondary flex items-center gap-1.5"
					type="button"
					onclick={cancelEdit}
					disabled={updateMutation.isPending}
				>
					<X size={16} aria-hidden="true" />
					Cancelar
				</button>
				<button
					class="btn-primary flex items-center gap-1.5"
					type="button"
					onclick={saveEdit}
					disabled={updateMutation.isPending}
				>
					<Save size={16} aria-hidden="true" />
					{updateMutation.isPending ? "Guardando..." : "Guardar"}
				</button>
			{:else if bankQuery.data}
				<button
					class="btn-secondary flex items-center gap-1.5"
					type="button"
					onclick={enterEdit}
				>
					<Pencil size={16} aria-hidden="true" />
					Editar
				</button>
				<button
					class="btn-secondary flex items-center gap-1.5"
					type="button"
					onclick={() => exportBankMarkdown(bankQuery.data)}
				>
					<FileDown size={16} aria-hidden="true" />
					Descargar MD
				</button>
			{/if}
		</div>
	</header>

	<section>
		{#if bankQuery.isLoading}
			<p class="m-0 text-zinc-600">Cargando banco...</p>
		{:else if bankQuery.error}
			<p class="m-0 text-red-700">
				{bankQuery.error?.messageOrDefault ?? ""}
			</p>
		{:else if bankQuery.data}
			{#if editing}
				<div class="grid gap-4">
					{#each draft as question, index (index)}
						<article class="panel-surface p-4 sm:p-5">
							<div class="mb-4 flex items-center justify-between gap-2">
								<h4 class="m-0 text-base font-medium text-black">
									Pregunta {index + 1}
								</h4>
							</div>

							<label class="grid gap-1.5">
								<span class="text-sm text-zinc-800">Enunciado (Markdown)</span>
								<MarkdownEditor bind:value={question.prompt} />
							</label>

							<div class="mt-4 grid gap-2">
								{#each question.options as _, optionIndex (optionIndex)}
									{@const correct = correctIndex(bankQuery.data.questions[index])}
									<div class="grid gap-1.5">
										<label class="grid gap-1.5">
											<span class="text-sm text-zinc-800">
												Opción {optionIndex + 1}
												{#if optionIndex === correct}
													<span class="text-emerald-700">
														(correcta)</span
													>
												{/if}
											</span>
											<input
												class="input-base"
												bind:value={question.options[optionIndex]}
											/>
										</label>
									</div>
								{/each}
							</div>

							<label class="mt-4 grid gap-1.5">
								<span class="text-sm text-zinc-800">Justificación (opcional)</span>
								<MarkdownEditor
									bind:value={question.justification}
									placeholder="Explica por qué la respuesta correcta lo es..."
								/>
							</label>
						</article>
					{/each}
				</div>
			{:else}
				<div class="grid gap-4">
					{#each bankQuery.data.questions as question, index (question.id)}
						<article class="panel-surface p-4 sm:p-5">
							<div class="mb-4 flex items-start gap-2">
								<span class="shrink-0 font-medium text-black">{index + 1}.</span>
								<div class="min-w-0 flex-1">
									<Markdown content={question.prompt} />
								</div>
							</div>
							<div class="grid gap-2">
								{#each question.options as option, optionIndex (optionIndex)}
									<div
										class="quiz-option"
										data-active={optionIndex === correctIndex(question)}
									>
										<Markdown content={option} inline />
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
						</article>
					{/each}
				</div>
			{/if}
		{/if}
	</section>
</section>
