<script lang="ts">
	import { goto } from "$app/navigation"
	import { toast } from "svelte-sonner"
	import { resolve } from "$app/paths"
	import { CircleStop } from "lucide-svelte"
	import { quizzesService } from "$lib/quizzes/service"
	import type { Quiz } from "$lib/quizzes/entity"
	import { Plus, Trash2, Eye, Pencil } from "lucide-svelte"
	import { useQuery, useMutation, useQueryClient } from "$lib/shared/http/tanstack"

	import EditQuizModal from "$lib/quizzes/components/EditQuizModal.svelte"
	import CreateQuizModal from "$lib/quizzes/components/CreateQuizModal.svelte"
	import QuizStatusBadge from "$lib/quizzes/components/QuizStatusBadge.svelte"
	import ConfirmActionModal from "$lib/shared/components/ConfirmActionModal.svelte"

	let { data } = $props()

	const queryClient = useQueryClient()

	const quizzesQuery = useQuery(() => ({
		queryKey: ["quizzes", "course", data.courseId],
		queryFn: () => quizzesService.list(data.courseId),
	}))

	const deleteQuizMutation = useMutation(() => ({
		mutationFn: (quizId: string) => quizzesService.remove(quizId),
		onSuccess: async () => {
			toast.success("Cuestionario eliminado correctamente.")
			await queryClient.invalidateQueries({
				queryKey: ["quizzes", "course", data.courseId],
			})
		},
		onError: (error) => {
			toast.error(error.messageOrDefault)
		},
	}))

	const closeAndPublishMutation = useMutation(() => ({
		mutationFn: (quizId: string) => quizzesService.closeAndPublish(quizId),
		onSuccess: async () => {
			toast.success("Cuestionario finalizado y resultados publicados.")
			await queryClient.invalidateQueries({
				queryKey: ["quizzes", "course", data.courseId],
			})
		},
		onError: (error) => {
			toast.error(error.messageOrDefault)
		},
	}))

	let showCreateModal = $state(false)
	let copiedJoinCode = $state<string | null>(null)
	let quizToEdit = $state<Quiz | null>(null)
	let quizToDelete = $state<{ id: string; title: string } | null>(null)

	const copyJoinCode = async (joinCode: string) => {
		await navigator.clipboard.writeText(joinCode)
		copiedJoinCode = joinCode
		setTimeout(() => {
			if (copiedJoinCode === joinCode) copiedJoinCode = null
		}, 1200)
		toast.success("Código copiado al portapapeles.")
	}

	const confirmDeleteQuiz = async () => {
		if (!quizToDelete) return
		await deleteQuizMutation.mutateAsync(quizToDelete.id)
		quizToDelete = null
	}

	const isActionPending = $derived(
		deleteQuizMutation.isPending || closeAndPublishMutation.isPending
	)
</script>

<section class="grid gap-4">
	<header>
		<div class="flex flex-wrap items-start justify-between gap-3">
			<div>
				<h3 class="sr-only">Cuestionarios</h3>
				<p class="m-0 mt-2 text-sm text-zinc-700">
					Programa evaluaciones, monitorea su estado y publica resultados cuando
					corresponda.
				</p>
			</div>
			<button
				class="btn-primary flex cursor-pointer items-center gap-1.5"
				type="button"
				onclick={() => (showCreateModal = true)}
			>
				<Plus size={16} aria-hidden="true" />
				Nuevo cuestionario
			</button>
		</div>
	</header>

	<section>
		{#if quizzesQuery.isLoading}
			<p class="m-0 text-zinc-600">Cargando cuestionarios...</p>
		{:else if quizzesQuery.error}
			<p class="m-0 text-red-700">
				{quizzesQuery.error?.messageOrDefault ?? ""}
			</p>
		{:else if !quizzesQuery.data?.length}
			<p class="m-0 text-zinc-600">Aún no existen cuestionarios para este curso.</p>
		{:else}
			<div class="grid grid-cols-[minmax(0,1fr)] gap-3 md:hidden">
				{#each quizzesQuery.data as quiz (quiz.id)}
					<div
						class="panel-surface flex cursor-pointer flex-col gap-3 p-4"
						role="button"
						tabindex="0"
						onclick={() =>
							goto(resolve(`/courses/${data.courseId}/quizzes/${quiz.id}/attempts`))}
						onkeydown={(e) => {
							if (e.key === "Enter")
								goto(
									resolve(`/courses/${data.courseId}/quizzes/${quiz.id}/attempts`)
								)
						}}
					>
						<div class="flex items-start justify-between gap-3">
							<div class="min-w-0">
								<p class="m-0 truncate font-medium text-zinc-900">{quiz.title}</p>
								<p class="m-0 mt-0.5 text-sm text-zinc-600">
									{quiz.kind.toDisplay()}
								</p>
							</div>
							<QuizStatusBadge resultsPublishedAt={quiz.resultsPublishedAt} />
						</div>
						<div class="flex flex-wrap items-center gap-1.5">
							<button
								type="button"
								class="btn-chip"
								title="Copiar código"
								onclick={(e) => {
									e.stopPropagation()
									void copyJoinCode(quiz.joinCode)
								}}
							>
								{copiedJoinCode === quiz.joinCode ? "Copiado" : quiz.joinCode}
							</button>
							<a
								class="icon-btn cursor-pointer"
								title="Ver intentos"
								href={resolve(
									`/courses/${data.courseId}/quizzes/${quiz.id}/attempts`
								)}
								onclick={(e) => e.stopPropagation()}
							>
								<Eye size={15} aria-hidden="true" />
							</a>
							<button
								class="icon-btn cursor-pointer"
								title="Editar"
								type="button"
								onclick={(e) => {
									e.stopPropagation()
									quizToEdit = quiz
								}}
								disabled={isActionPending}
							>
								<Pencil size={15} aria-hidden="true" />
							</button>
							{#if !quiz.resultsPublishedAt}
								<button
									class="icon-btn cursor-pointer border-amber-300 text-amber-800 hover:bg-amber-50"
									title="Finalizar y publicar"
									type="button"
									onclick={(e) => {
										e.stopPropagation()
										closeAndPublishMutation.mutate(quiz.id)
									}}
									disabled={isActionPending || !!quiz.resultsPublishedAt}
								>
									<CircleStop size={15} aria-hidden="true" />
								</button>
							{/if}
							<button
								class="icon-btn icon-btn-danger cursor-pointer"
								title="Eliminar"
								type="button"
								onclick={(e) => {
									e.stopPropagation()
									quizToDelete = { id: quiz.id, title: quiz.title }
								}}
								disabled={isActionPending}
							>
								<Trash2 size={15} aria-hidden="true" />
							</button>
						</div>
					</div>
				{/each}
			</div>
			<div class="hidden overflow-x-auto md:block">
				<table class="min-w-full border-collapse text-sm">
					<thead class="table-head">
						<tr>
							<th class="px-3 py-2 text-left font-medium">Título</th>
							<th class="px-3 py-2 text-left font-medium">Tipo</th>
							<th class="px-3 py-2 text-left font-medium">Estado</th>
							<th class="px-3 py-2 text-left font-medium">Código de acceso</th>
							<th class="px-3 py-2 text-left font-medium">Acciones</th>
						</tr>
					</thead>
					<tbody>
						{#each quizzesQuery.data as quiz (quiz.id)}
							<tr
								class="row-hover table-row cursor-pointer"
								onclick={() =>
									goto(
										resolve(
											`/courses/${data.courseId}/quizzes/${quiz.id}/attempts`
										)
									)}
							>
								<td class="px-3 py-2 text-zinc-900">{quiz.title}</td>
								<td class="px-3 py-2 text-zinc-700">{quiz.kind.toDisplay()}</td>
								<td class="px-3 py-2 text-zinc-700">
									<QuizStatusBadge resultsPublishedAt={quiz.resultsPublishedAt} />
								</td>
								<td class="px-3 py-2 text-zinc-700">
									<button
										type="button"
										class="btn-chip"
										title="Copiar código"
										onclick={(e) => {
											e.stopPropagation()
											void copyJoinCode(quiz.joinCode)
										}}
									>
										{copiedJoinCode === quiz.joinCode
											? "Copiado"
											: quiz.joinCode}
									</button>
								</td>
								<td class="px-3 py-2">
									<div class="flex items-center gap-1">
										<a
											class="icon-btn cursor-pointer"
											title="Ver intentos"
											onclick={(e) => e.stopPropagation()}
											href={resolve(
												`/courses/${data.courseId}/quizzes/${quiz.id}/attempts`
											)}
										>
											<Eye size={15} aria-hidden="true" /></a
										>
										<button
											class="icon-btn cursor-pointer"
											title="Editar"
											type="button"
											onclick={(e) => {
												e.stopPropagation()
												quizToEdit = quiz
											}}
											disabled={isActionPending}
										>
											<Pencil size={15} aria-hidden="true" /></button
										>
										{#if !quiz.resultsPublishedAt}
											<button
												class="icon-btn cursor-pointer border-amber-300 text-amber-800 hover:bg-amber-50"
												title="Finalizar y publicar"
												type="button"
												onclick={(e) => {
													e.stopPropagation()
													closeAndPublishMutation.mutate(quiz.id)
												}}
												disabled={isActionPending ||
													!!quiz.resultsPublishedAt}
											>
												<CircleStop size={15} aria-hidden="true" /></button
											>
										{/if}
										<button
											class="icon-btn icon-btn-danger cursor-pointer"
											title="Eliminar"
											type="button"
											onclick={(e) => {
												e.stopPropagation()
												quizToDelete = { id: quiz.id, title: quiz.title }
											}}
											disabled={isActionPending}
										>
											<Trash2 size={15} aria-hidden="true" /></button
										>
									</div>
								</td>
							</tr>
						{/each}
					</tbody>
				</table>
			</div>
		{/if}
	</section>

	<CreateQuizModal
		open={showCreateModal}
		courseId={data.courseId}
		onclose={() => (showCreateModal = false)}
		onsuccess={() => (showCreateModal = false)}
	/>

	{#if quizToEdit}
		{#key quizToEdit.id}
			<EditQuizModal
				quiz={quizToEdit}
				courseId={data.courseId}
				onclose={() => (quizToEdit = null)}
			/>
		{/key}
	{/if}

	<ConfirmActionModal
		open={!!quizToDelete}
		title="Eliminar cuestionario"
		message={quizToDelete ? `Se eliminará el cuestionario ${quizToDelete.title}.` : ""}
		confirmLabel="Eliminar"
		isPending={deleteQuizMutation.isPending}
		onCancel={() => (quizToDelete = null)}
		onConfirm={confirmDeleteQuiz}
	/>
</section>
