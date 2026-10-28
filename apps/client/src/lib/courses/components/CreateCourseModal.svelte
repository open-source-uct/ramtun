<script lang="ts">
	import type { SubmitEventHandler } from "@formisch/svelte"
	import type { CreateCourseDTO, CreateCourseDTOSchema } from "$lib/courses/dtos"

	import { toast } from "svelte-sonner"
	import { fade, scale } from "svelte/transition"
	import { coursesService } from "../service"
	import { Plus, RefreshCw, X } from "lucide-svelte"
	import { createCourseDTOSchema } from "$lib/courses/dtos"
	import { queryClient, useMutation } from "$lib/shared/http/tanstack"
	import { createForm, Field, Form, reset } from "@formisch/svelte"

	interface CreateCourseModalProps {
		open: boolean
		onclose: () => void
		onsuccess: () => void
	}

	let { open, onclose, onsuccess }: CreateCourseModalProps = $props()

	const mutation = useMutation(() => ({
		mutationFn: (input: CreateCourseDTO) => coursesService.create(input),
		onSuccess: (created) => {
			toast.success(`Curso ${created.code} creado correctamente.`)
			queryClient.invalidateQueries({ queryKey: ["courses"] })
			onsuccess()
		},
		onError: (error) => {
			toast.error(error.messageOrDefault)
		},
	}))

	const formInitialInput = {
		name: "",
		code: "",
		year: new Date().getFullYear(),
	}

	const form = createForm({
		schema: createCourseDTOSchema,
		initialInput: formInitialInput,
	})

	const handleSubmit: SubmitEventHandler<CreateCourseDTOSchema> = async (output) => {
		await mutation.mutateAsync(output)
		reset(form, { initialInput: formInitialInput })
	}
</script>

{#if open}
	<div
		class="fixed inset-0 z-50 flex items-center justify-center bg-(--scrim) p-4"
		role="dialog"
		aria-modal="true"
		tabindex="-1"
		transition:fade={{ duration: 180 }}
		onclick={() => onclose()}
		onkeydown={(e) => {
			if (e.key === "Escape") {
				onclose()
				reset(form, { initialInput: formInitialInput })
			}
		}}
	>
		<section
			class="panel-elevated w-full max-w-xl p-5 sm:p-6"
			role="presentation"
			tabindex="-1"
			transition:scale={{ duration: 190, start: 0.98 }}
			onclick={(e) => e.stopPropagation()}
		>
			<div class="mb-3 flex items-center justify-between gap-2">
				<h3 class="m-0 text-lg text-black">Crear curso</h3>
				<button class="btn-close" type="button" onclick={onclose} aria-label="Cerrar">
					<X size={18} aria-hidden="true" />
				</button>
			</div>

			<Form of={form} onsubmit={handleSubmit} class="form-stack">
				<Field of={form} path={["name"]}>
					{#snippet children(field)}
						<label class="grid gap-1.5">
							<span class="text-sm text-zinc-800">Nombre</span>
							<input
								{...field.props}
								class="input-base"
								value={field.input ?? ""}
								placeholder="Ej: Matematica Discreta"
							/>
							{#if field.errors?.[0]}
								<span class="text-sm text-red-700">{field.errors[0]}</span>
							{/if}
						</label>
					{/snippet}
				</Field>

				<div class="form-grid-2">
					<Field of={form} path={["code"]}>
						{#snippet children(field)}
							<label class="grid gap-1.5">
								<span class="text-sm text-zinc-800">Código</span>
								<input
									{...field.props}
									class="input-base"
									value={field.input ?? ""}
									placeholder="Ej: MAT123"
								/>
								{#if field.errors?.[0]}
									<span class="text-sm text-red-700">{field.errors[0]}</span>
								{/if}
							</label>
						{/snippet}
					</Field>

					<Field of={form} path={["year"]}>
						{#snippet children(field)}
							<label class="grid gap-1.5">
								<span class="text-sm text-zinc-800">Año</span>
								<input
									{...field.props}
									class="input-base"
									type="number"
									value={field.input ?? ""}
								/>
								{#if field.errors?.[0]}
									<span class="text-sm text-red-700">{field.errors[0]}</span>
								{/if}
							</label>
						{/snippet}
					</Field>
				</div>

				<div class="flex justify-end gap-2">
					<button class="btn-tertiary" type="button" onclick={onclose}>Cancelar</button>
					<button class="btn-primary flex items-center gap-1.5" type="submit">
						{#if mutation.isPending}
							<RefreshCw size={16} class="animate-spin" aria-hidden="true" />
						{:else}
							<Plus size={16} aria-hidden="true" />
						{/if}
						{mutation.isPending ? "Creando..." : "Crear curso"}
					</button>
				</div>
			</Form>
		</section>
	</div>
{/if}
