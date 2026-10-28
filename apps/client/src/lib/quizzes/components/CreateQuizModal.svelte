<script lang="ts">
	import type * as v from "valibot"
	import type { SubmitEventHandler } from "@formisch/svelte"
	import type { CertaintyTableDTO, CreateQuizDTO, CreateQuizDTOSchema } from "$lib/quizzes/dtos"

	import { toast } from "svelte-sonner"
	import { Plus, X } from "lucide-svelte"
	import { untrack } from "svelte"
	import { fade, scale } from "svelte/transition"
	import { getInput, setInput } from "@formisch/svelte"
	import { queryClient, useMutation } from "$lib/shared/http/tanstack"
	import { createForm, Field, Form, reset } from "@formisch/svelte"

	import { quizzesService } from "../service"
	import { CertaintyValue } from "$lib/shared/value-objects/certainty.value"
	import { createQuizDTOSchema } from "$lib/quizzes/dtos"

	import BankSelector from "$lib/quizzes/components/BankSelector.svelte"

	interface CreateQuizModalProps {
		open: boolean
		courseId: string
		onclose: () => void
		onsuccess: () => void
	}

	let { open, courseId, onclose, onsuccess }: CreateQuizModalProps = $props()

	const mutation = useMutation(() => ({
		mutationFn: (data: CreateQuizDTO) => {
			const startsAtUtc = new Date(data.startsAt).toISOString()
			return quizzesService.create({ ...data, startsAt: startsAtUtc })
		},
		onSuccess: async (created) => {
			toast.success(`Cuestionario ${created.title} creado.`)
			await queryClient.invalidateQueries({
				queryKey: ["quizzes", "course", courseId],
			})
		},
		onError: (error) => {
			toast.error(error.messageOrDefault)
		},
	}))

	const defaultCertainty: CertaintyTableDTO = {
		low: { correct: 1, incorrect: 0 },
		medium: { correct: 2, incorrect: -2 },
		high: { correct: 3, incorrect: -4 },
	}

	const initialInput = untrack(() => {
		const now = new Date()
		const roundedUp5 = (m: number) => (Math.floor(m / 5) + 1) * 5
		const initialStart = new Date(
			now.getFullYear(),
			now.getMonth(),
			now.getDate(),
			now.getHours(),
			roundedUp5(now.getMinutes())
		)
		const pad = (n: number) => String(n).padStart(2, "0")
		const localString =
			`${initialStart.getFullYear()}-${pad(initialStart.getMonth() + 1)}-${pad(initialStart.getDate())}` +
			`T${pad(initialStart.getHours())}:${pad(initialStart.getMinutes())}`

		return {
			courseId,
			title: "",
			kind: "traditional" as const,
			startsAt: localString,
			attemptDurationMinutes: "30",
			questionCount: "10",
			bankIds: [] as string[],
			certaintyConfig: null,
			resultsMultipleViews: false,
		} satisfies v.InferInput<typeof createQuizDTOSchema>
	})

	const form = createForm({ schema: createQuizDTOSchema, initialInput, validate: "blur" })

	const kind = $derived(
		(getInput(form, { path: ["kind"] as any }) ?? "traditional") as "traditional" | "certainty"
	)
	const certaintyConfigValue = $derived(
		(getInput(form, { path: ["certaintyConfig"] as any }) ?? null) as CertaintyTableDTO | null
	)

	$effect(() => {
		if (kind === "traditional" && certaintyConfigValue !== null) {
			setInput(form, { path: ["certaintyConfig"] as any, input: null } as never)
		} else if (kind === "certainty" && !certaintyConfigValue) {
			setInput(form, { path: ["certaintyConfig"] as any, input: defaultCertainty } as never)
		}
	})

	const toggleBank = (bankId: string, selected: boolean) => {
		const current = (getInput(form, { path: ["bankIds"] as any }) ?? []) as string[]
		const next = selected ? [...current, bankId] : current.filter((id) => id !== bankId)
		setInput(form, { path: ["bankIds"] as any, input: next } as never)
	}

	const updateCertainty = (
		level: keyof CertaintyTableDTO,
		field: "correct" | "incorrect",
		value: number
	) => {
		const current = certaintyConfigValue ?? defaultCertainty
		setInput(form, {
			path: ["certaintyConfig"] as any,
			input: {
				...current,
				[level]: { ...current[level], [field]: value },
			},
		} as never)
	}

	const handleSubmit: SubmitEventHandler<CreateQuizDTOSchema> = async (output) => {
		await mutation.mutateAsync(output)
		reset(form, { initialInput })
		onsuccess()
	}

	const modalOnClose = () => {
		onclose()
		reset(form, { initialInput })
	}
</script>

{#if open}
	<div
		class="fixed inset-0 z-50 flex items-center justify-center bg-(--scrim) p-4"
		role="dialog"
		aria-modal="true"
		tabindex="-1"
		transition:fade={{ duration: 180 }}
		onclick={() => modalOnClose()}
		onkeydown={(e) => {
			if (e.key === "Escape") modalOnClose()
		}}
	>
		<section
			class="panel-elevated w-full max-w-xl p-5"
			role="presentation"
			tabindex="-1"
			transition:scale={{ duration: 190, start: 0.98 }}
			onclick={(e) => e.stopPropagation()}
		>
			<div class="mb-3 flex items-center justify-between gap-2">
				<h4 class="m-0 text-base text-black">Crear cuestionario</h4>
				<button class="btn-close" type="button" onclick={modalOnClose} aria-label="Cerrar">
					<X size={18} aria-hidden="true" />
				</button>
			</div>

			<Form of={form} onsubmit={handleSubmit} class="form-stack">
				<Field of={form} path={["title"]}>
					{#snippet children(field)}
						<label class="grid gap-1.5">
							<span class="text-sm text-zinc-800">Título</span>
							<input {...field.props} class="input-base" value={field.input ?? ""} />
							<span
								class="text-sm text-red-700"
								class:invisible={!field.errors?.[0]}
								aria-live="polite"
							>
								{field.errors?.[0] ?? " "}
							</span>
						</label>
					{/snippet}
				</Field>

				<div class="form-grid-2 grid-cols-2">
					<Field of={form} path={["kind"]}>
						{#snippet children(field)}
							<label class="grid gap-1.5">
								<span class="text-sm text-zinc-800">Tipo</span>
								<select
									{...field.props}
									class="input-base"
									value={field.input ?? "traditional"}
								>
									<option value="traditional">Tradicional</option>
									<option value="certainty">Certeza</option>
								</select>
								<span
									class="text-sm text-red-700"
									class:invisible={!field.errors?.[0]}
									aria-live="polite"
								>
									{field.errors?.[0] ?? " "}
								</span>
							</label>
						{/snippet}
					</Field>

					<Field of={form} path={["questionCount"]}>
						{#snippet children(field)}
							<label class="grid gap-1.5">
								<span class="text-sm text-zinc-800">Cantidad preguntas</span>
								<input
									{...field.props}
									type="number"
									min="1"
									max="100"
									class="input-base"
									value={field.input ?? 10}
								/>
								<span
									class="text-sm text-red-700"
									class:invisible={!field.errors?.[0]}
									aria-live="polite"
								>
									{field.errors?.[0] ?? " "}
								</span>
							</label>
						{/snippet}
					</Field>
				</div>

				<div class="form-grid-2 grid-cols-2">
					<Field of={form} path={["startsAt"]}>
						{#snippet children(field)}
							<label class="grid gap-1.5">
								<span class="text-sm text-zinc-800">Inicio</span>
								<input
									{...field.props}
									type="datetime-local"
									class="input-base h-10"
									value={field.input ?? ""}
								/>
								<span
									class="text-sm text-red-700"
									class:invisible={!field.errors?.[0]}
									aria-live="polite"
								>
									{field.errors?.[0] ?? " "}
								</span>
							</label>
						{/snippet}
					</Field>

					<Field of={form} path={["attemptDurationMinutes"]}>
						{#snippet children(field)}
							<label class="grid gap-1.5">
								<span class="text-sm text-zinc-800">Duración (min)</span>
								<input
									{...field.props}
									type="number"
									min="1"
									max="240"
									class="input-base"
									value={field.input ?? 30}
								/>
								<span
									class="text-sm text-red-700"
									class:invisible={!field.errors?.[0]}
									aria-live="polite"
								>
									{field.errors?.[0] ?? " "}
								</span>
							</label>
						{/snippet}
					</Field>
				</div>

				<Field of={form} path={["bankIds"]}>
					{#snippet children(field)}
						<div class="grid gap-2">
							<span class="text-sm text-zinc-800">Bancos fuente</span>
							<BankSelector
								{courseId}
								selectedBankIds={field.input ?? []}
								onchange={toggleBank}
							/>
							{#if field.errors?.[0]}
								<span class="text-sm text-red-700">{field.errors[0]}</span>
							{/if}
						</div>
					{/snippet}
				</Field>

				<Field of={form} path={["resultsMultipleViews"]}>
					{#snippet children(field)}
						<label class="flex cursor-pointer items-start gap-2.5">
							<input
								{...field.props}
								type="checkbox"
								class="mt-0.5"
								checked={field.input ?? false}
							/>
							<span class="grid gap-0.5">
								<span class="text-sm text-zinc-800">
									Permitir revisar los resultados más de una vez
								</span>
								<span class="text-xs text-zinc-500">
									Si no la marcas, cada estudiante ve sus resultados una sola vez.
								</span>
							</span>
						</label>
					{/snippet}
				</Field>

				{#if kind === "certainty"}
					<div class="grid gap-3">
						<div class="flex items-center justify-between">
							<span class="text-sm text-zinc-800">Tabla de certeza</span>
							<span class="text-xs text-zinc-500">Puntos por respuesta</span>
						</div>
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
									{#each ["low", "medium", "high"] as const as level (level)}
										<tr class="border-b border-zinc-200/70 last:border-b-0">
											<td class="px-3 py-2 text-zinc-700">
												{CertaintyValue.format(level)}
											</td>
											<td class="px-2 py-1.5">
												<input
													type="number"
													class="input-base h-10"
													value={certaintyConfigValue?.[level].correct ??
														0}
													oninput={(e) =>
														updateCertainty(
															level,
															"correct",
															Number(e.currentTarget.value)
														)}
												/>
											</td>
											<td class="px-2 py-1.5">
												<input
													type="number"
													class="input-base h-10"
													value={certaintyConfigValue?.[level]
														.incorrect ?? 0}
													oninput={(e) =>
														updateCertainty(
															level,
															"incorrect",
															Number(e.currentTarget.value)
														)}
												/>
											</td>
										</tr>
									{/each}
								</tbody>
							</table>
						</div>
					</div>
				{/if}

				<div class="flex justify-end gap-2">
					<button class="btn-tertiary" type="button" onclick={modalOnClose}
						>Cancelar</button
					>
					<button
						class="btn-primary flex items-center gap-1.5"
						type="submit"
						disabled={mutation.isPending}
					>
						<Plus size={16} aria-hidden="true" />
						{mutation.isPending ? "Creando..." : "Crear cuestionario"}
					</button>
				</div>
			</Form>
		</section>
	</div>
{/if}
