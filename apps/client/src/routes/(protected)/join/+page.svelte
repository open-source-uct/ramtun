<script lang="ts">
	import type { JoinCodeDTOSchema } from "$lib/quizzes/dtos"
	import type { SubmitEventHandler } from "@formisch/svelte"

	import { goto } from "$app/navigation"
	import { toast } from "svelte-sonner"
	import { Search } from "lucide-svelte"
	import { useMutation } from "$lib/shared/http/tanstack"
	import { quizzesService } from "$lib/quizzes/service"
	import { joinCodeDTOSchema } from "$lib/quizzes/dtos"
	import { createForm, Field, Form, reset } from "@formisch/svelte"

	const form = createForm({
		schema: joinCodeDTOSchema,
		validate: "blur",
	})

	const mutation = useMutation(() => ({
		mutationFn: (code: string) => quizzesService.joinByCode(code),
		onSuccess: async (_, code) => {
			reset(form)
			await goto(`/join/lobby?joinCode=${encodeURIComponent(code)}`)
		},
		onError: (error) => {
			toast.error(error.messageOrDefault)
		},
	}))

	const handleSubmit: SubmitEventHandler<JoinCodeDTOSchema> = async (input) => {
		await mutation.mutateAsync(input.joinCode)
	}
</script>

<section class="grid gap-5">
	<header>
		<h2 class="mt-2 mb-0 text-2xl text-black">Unirse a un cuestionario</h2>
	</header>

	<section>
		<Form of={form} onsubmit={handleSubmit}>
			<div class="flex flex-col gap-3 sm:flex-row sm:items-start">
				<div class="flex-1">
					<Field of={form} path={["joinCode"]}>
						{#snippet children(field)}
							<label class="grid gap-1.5">
								<input
									class="input-base"
									type="text"
									aria-label="Código de cuestionario"
									{...field.props}
									value={field.input}
									placeholder="Ingrese un código del cuestionario. Ej: ABC1234"
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

				<button
					class="btn-primary flex shrink-0 cursor-pointer items-center gap-1.5"
					type="submit"
					disabled={mutation.isPending}
				>
					<Search size={16} aria-hidden="true" />
					{mutation.isPending ? "Validando..." : "Entrar"}
				</button>
			</div>
		</Form>
	</section>
</section>
