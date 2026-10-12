<script lang="ts">
	import type { CreateQuestionBankInput } from "$lib/banks/banks.dtos"

	import { goto } from "$app/navigation"
	import { toast } from "svelte-sonner"
	import { resolve } from "$app/paths"
	import { DateValue } from "$lib/shared/value-objects/date.value"
	import { Download, Plus, Trash2 } from "lucide-svelte"
	import { banksService } from "$lib/banks/banks.service"
	import { exportBankMarkdown } from "$lib/banks/utils"
	import { useQuery, useMutation, useQueryClient } from "$lib/shared/http/tanstack"

	import BankUploadModal from "$lib/banks/components/BankUploadModal.svelte"
	import ConfirmActionModal from "$lib/shared/components/ConfirmActionModal.svelte"

	let { data } = $props()

	const queryClient = useQueryClient()

	const banksQuery = useQuery(() => ({
		queryKey: ["banks", data.courseId],
		queryFn: () => banksService.listByCourse(data.courseId),
	}))

	let showCreateModal = $state(false)
	let bankToDelete = $state<{ id: string; name: string } | null>(null)

	const uploadBankMutation = useMutation(() => ({
		mutationFn: (input: CreateQuestionBankInput) => banksService.create(input),
		onSuccess: async () => {
			toast.success("Banco de preguntas creado correctamente.")
			await queryClient.invalidateQueries({ queryKey: ["banks", data.courseId] })
		},
		onError: (error) => {
			toast.error(error.messageOrDefault)
		},
	}))

	const deleteBankMutation = useMutation(() => ({
		mutationFn: (bankId: string) => banksService.softDelete(bankId),
		onSuccess: async () => {
			bankToDelete = null
			await queryClient.invalidateQueries({ queryKey: ["banks", data.courseId] })
		},
		onError: (error) => {
			toast.error(error.messageOrDefault)
		},
	}))

	const confirmDeleteBank = () => {
		if (!bankToDelete) return
		deleteBankMutation.mutate(bankToDelete.id)
	}
</script>

<section class="grid gap-4">
	<header>
		<div class="flex flex-wrap items-start justify-between gap-3">
			<div>
				<h3 class="sr-only">Bancos de preguntas</h3>
				<p class="m-0 mt-2 text-sm text-zinc-700">
					Sube y gestiona bancos de preguntas en formato Markdown.
				</p>
			</div>
			<button
				class="btn-primary flex items-center gap-1.5"
				type="button"
				onclick={() => (showCreateModal = true)}
			>
				<Plus size={16} aria-hidden="true" />
				Nuevo banco
			</button>
		</div>
	</header>

	<section>
		{#if banksQuery.isLoading}
			<p class="m-0 text-zinc-600">Cargando bancos...</p>
		{:else if banksQuery.isError}
			<p class="m-0 text-red-700">Error al cargar los bancos.</p>
		{:else if !banksQuery.data?.length}
			<div class="panel-surface py-10 text-center">
				<p class="mb-2 text-zinc-600">No hay bancos de preguntas.</p>
				<p class="text-sm text-zinc-500">
					Sube un archivo Markdown para crear el primer banco.
				</p>
			</div>
		{:else}
			<div class="grid gap-3 md:hidden">
				{#each banksQuery.data as bank (bank.id)}
					<div
						class="panel-surface flex cursor-pointer items-center justify-between gap-3 p-4"
						role="button"
						tabindex="0"
						onclick={() => goto(resolve(`/courses/${data.courseId}/banks/${bank.id}`))}
						onkeydown={(e) => {
							if (e.key === "Enter")
								goto(resolve(`/courses/${data.courseId}/banks/${bank.id}`))
						}}
					>
						<div class="min-w-0">
							<p class="m-0 font-medium text-zinc-900">{bank.name}</p>
							<p class="m-0 mt-1 text-sm text-zinc-600">
								{bank.questions.length} preguntas · {DateValue.format(
									bank.created_at ?? bank.createdAt ?? ""
								)}
							</p>
						</div>
						<div class="flex shrink-0 items-center gap-1">
							<button
								class="icon-btn cursor-pointer"
								title="Descargar MD"
								type="button"
								onclick={(e) => {
									e.stopPropagation()
									exportBankMarkdown(bank)
								}}
							>
								<Download size={15} aria-hidden="true" />
							</button>
							<button
								class="icon-btn icon-btn-danger cursor-pointer"
								title="Eliminar"
								type="button"
								onclick={(e) => {
									e.stopPropagation()
									bankToDelete = { id: bank.id, name: bank.name }
								}}
								disabled={deleteBankMutation.isPending}
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
							<th class="px-3 py-2 text-left font-medium">Nombre</th>
							<th class="px-3 py-2 text-left font-medium">Preguntas</th>
							<th class="px-3 py-2 text-left font-medium">Creado</th>
							<th class="px-3 py-2 text-left font-medium">Acciones</th>
						</tr>
					</thead>
					<tbody>
						{#each banksQuery.data as bank (bank.id)}
							<tr
								class="row-hover table-row cursor-pointer"
								onclick={() =>
									goto(resolve(`/courses/${data.courseId}/banks/${bank.id}`))}
							>
								<td class="px-3 py-2 text-zinc-900">{bank.name}</td>
								<td class="px-3 py-2 text-zinc-700">{bank.questions.length}</td>
								<td class="px-3 py-2 text-zinc-700"
									>{DateValue.format(bank.created_at ?? bank.createdAt ?? "")}</td
								>
								<td class="px-3 py-2">
									<div class="flex items-center gap-1">
										<button
											class="icon-btn cursor-pointer"
											title="Descargar MD"
											type="button"
											onclick={(e) => {
												e.stopPropagation()
												exportBankMarkdown(bank)
											}}
										>
											<Download size={15} aria-hidden="true" />
										</button>
										<button
											class="icon-btn icon-btn-danger"
											title="Eliminar"
											type="button"
											onclick={(e) => {
												e.stopPropagation()
												bankToDelete = { id: bank.id, name: bank.name }
											}}
											disabled={deleteBankMutation.isPending}
										>
											<Trash2 size={15} aria-hidden="true" />
										</button>
									</div>
								</td>
							</tr>
						{/each}
					</tbody>
				</table>
			</div>
		{/if}
	</section>
</section>

<BankUploadModal
	open={showCreateModal}
	courseId={data.courseId}
	onclose={() => (showCreateModal = false)}
	onsuccess={() => (showCreateModal = false)}
	mutation={uploadBankMutation}
/>

<ConfirmActionModal
	open={!!bankToDelete}
	title="Eliminar banco"
	message={bankToDelete ? `Se eliminara el banco ${bankToDelete.name}.` : ""}
	confirmLabel="Eliminar"
	isPending={deleteBankMutation.isPending}
	onCancel={() => (bankToDelete = null)}
	onConfirm={confirmDeleteBank}
/>
