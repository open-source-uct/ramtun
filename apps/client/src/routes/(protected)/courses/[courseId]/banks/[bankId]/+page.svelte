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
	import BankQuestionsEditor, {
		type DraftQuestion,
	} from "$lib/banks/components/BankQuestionsEditor.svelte"
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

	let editing = $state(false)
	let draftName = $state("")
	let draft = $state<DraftQuestion[]>([])

	const correctIndex = (question: { answer_index?: number; answerIndex?: number }) =>
		question.answer_index ?? question.answerIndex

	const enterEdit = () => {
		const bank = bankQuery.data
		if (!bank) return

		draftName = bank.name
		draft = bank.questions.map((question) => ({
			key: crypto.randomUUID(),
			prompt: question.prompt,
			options: [...question.options],
			answerIndex: correctIndex(question) ?? 0,
			justification: question.justification ?? "",
		}))
		editing = true
	}

	const cancelEdit = () => {
		editing = false
		draft = []
	}

	// Primer problema que impediría guardar; el servidor valida lo mismo.
	const draftProblem = $derived.by(() => {
		if (!draftName.trim()) return "El banco necesita un nombre."
		if (draftName.trim().length > 120) return "El nombre no puede superar los 120 caracteres."
		if (draft.length === 0) return "El banco necesita al menos una pregunta."

		for (const [index, question] of draft.entries()) {
			if (!question.prompt.trim()) return `La pregunta ${index + 1} no tiene enunciado.`
			if (question.options.some((option) => !option.trim()))
				return `La pregunta ${index + 1} tiene opciones vacías.`
		}

		return null
	})

	const updateMutation = useMutation(() => ({
		mutationFn: () =>
			banksService.update(data.bankId, {
				name: draftName.trim(),
				questions: draft.map((question) => ({
					prompt: question.prompt,
					options: question.options,
					answerIndex: question.answerIndex,
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
		if (updateMutation.isPending || draftProblem) return
		updateMutation.mutate()
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
			{#if editing}
				<label class="grid gap-1.5">
					<span class="text-sm text-zinc-800">Nombre del banco</span>
					<input
						class="input-base w-full sm:w-96"
						bind:value={draftName}
						maxlength="120"
						disabled={updateMutation.isPending}
					/>
				</label>
			{:else}
				<div class="flex min-w-0 flex-wrap items-baseline gap-x-3 gap-y-1">
					<Title size="page">{bankQuery.data?.name ?? "Banco"}</Title>
					<p class="m-0 text-sm text-zinc-700">
						{bankQuery.data?.questions.length ?? 0} preguntas
					</p>
				</div>
			{/if}
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
					disabled={updateMutation.isPending || !!draftProblem}
					title={draftProblem ?? undefined}
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
				<BankQuestionsEditor bind:questions={draft} disabled={updateMutation.isPending} />
				{#if draftProblem}
					<p class="mt-3 mb-0 text-sm text-red-700" role="status">{draftProblem}</p>
				{/if}
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
