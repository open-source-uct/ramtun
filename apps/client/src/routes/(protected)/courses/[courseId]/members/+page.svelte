<script lang="ts">
	import type { CourseMember } from "$lib/courses/entity"

	import { toast } from "svelte-sonner"
	import { authStore } from "$lib/auth/store.svelte"
	import { RoleValue } from "$lib/shared/value-objects/role.value"
	import { Plus, Trash2 } from "lucide-svelte"
	import { coursesService } from "$lib/courses/service"
	import { useQuery, useMutation, useQueryClient } from "$lib/shared/http/tanstack"

	import AddMemberModal from "$lib/courses/components/AddMemberModal.svelte"
	import ConfirmActionModal from "$lib/shared/components/ConfirmActionModal.svelte"

	let { data } = $props()

	let showAddModal = $state(false)
	let memberToRemove = $state<CourseMember | null>(null)

	const queryClient = useQueryClient()

	const membersQuery = useQuery(() => ({
		queryKey: ["course-members", data.courseId],
		queryFn: () => coursesService.listMembers(data.courseId),
	}))

	const removeMemberMutation = useMutation(() => ({
		mutationFn: (userId: string) => coursesService.removeMember(data.courseId, userId),
		onSuccess: async () => {
			toast.success("Miembro removido correctamente.")
			await queryClient.invalidateQueries({
				queryKey: ["course-members", data.courseId],
			})
		},
		onError: (error) => {
			toast.error(error.messageOrDefault)
		},
	}))

	const confirmRemoveMember = async () => {
		if (!memberToRemove) return
		await removeMemberMutation.mutateAsync(memberToRemove.userId)
		memberToRemove = null
	}
</script>

<section class="grid gap-4">
	<header>
		<div class="flex flex-wrap items-start justify-between gap-3">
			<div>
				<h3 class="sr-only">Miembros</h3>
				<p class="mt-2 mb-0 text-sm text-zinc-700">Agrega o retira participantes.</p>
			</div>
			<button
				class="btn-primary flex items-center gap-1.5"
				type="button"
				onclick={() => (showAddModal = true)}
			>
				<Plus size={16} aria-hidden="true" />
				Agregar miembro
			</button>
		</div>
	</header>

	<section>
		{#if membersQuery.isLoading}
			<p class="m-0 text-zinc-600">Cargando miembros...</p>
		{:else if membersQuery.error}
			<p class="m-0 text-red-700">
				{membersQuery.error?.messageOrDefault ?? ""}
			</p>
		{:else if !membersQuery.data?.length}
			<p class="notice notice-warn m-0">El curso aún no tiene miembros.</p>
		{:else}
			<div class="grid gap-3 md:hidden">
				{#each membersQuery.data as member (member.userId)}
					<div class="panel-surface flex items-center justify-between gap-3 p-4">
						<div class="min-w-0">
							<p class="m-0 font-medium text-zinc-900">{member.username}</p>
							<p class="m-0 mt-0.5 text-sm text-zinc-600">
								{member.name} · {RoleValue.format(member.role)}
							</p>
						</div>
						{#if member.userId !== authStore.user?.id}
							<button
								class="icon-btn icon-btn-danger shrink-0"
								title="Quitar"
								type="button"
								disabled={removeMemberMutation.isPending}
								onclick={() => {
									memberToRemove = member
								}}
							>
								<Trash2 size={15} aria-hidden="true" />
							</button>
						{/if}
					</div>
				{/each}
			</div>
			<div class="hidden overflow-x-auto md:block">
				<table class="min-w-full border-collapse text-sm">
					<thead class="table-head">
						<tr>
							<th class="px-3 py-2 text-left font-medium">Usuario</th>
							<th class="px-3 py-2 text-left font-medium">Nombre</th>
							<th class="px-3 py-2 text-left font-medium">Rol</th>
							<th class="px-3 py-2 text-left font-medium">Acciones</th>
						</tr>
					</thead>
					<tbody>
						{#each membersQuery.data as member (member.userId)}
							<tr class="table-row">
								<td class="px-3 py-2 font-medium text-zinc-900"
									>{member.username}</td
								>
								<td class="px-3 py-2 text-zinc-800">{member.name}</td>
								<td class="px-3 py-2 text-zinc-700"
									>{RoleValue.format(member.role)}</td
								>
								<td class="px-3 py-2">
									{#if member.userId !== authStore.user?.id}
										<button
											class="icon-btn icon-btn-danger"
											title="Quitar"
											type="button"
											disabled={removeMemberMutation.isPending}
											onclick={() => {
												memberToRemove = member
											}}
										>
											<Trash2 size={15} aria-hidden="true" />
										</button>
									{/if}
								</td>
							</tr>
						{/each}
					</tbody>
				</table>
			</div>
		{/if}
	</section>

	<AddMemberModal
		open={showAddModal}
		oncancel={() => (showAddModal = false)}
		courseId={data.courseId}
	/>

	<ConfirmActionModal
		open={!!memberToRemove}
		title="Quitar miembro"
		message={memberToRemove ? `Se quitara a ${memberToRemove.username} de este curso.` : ""}
		confirmLabel="Quitar"
		isPending={removeMemberMutation.isPending}
		onCancel={() => (memberToRemove = null)}
		onConfirm={confirmRemoveMember}
	/>
</section>
