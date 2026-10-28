<script lang="ts">
	import { page } from "$app/state"
	import { resolve } from "$app/paths"
	import { goto } from "$app/navigation"
	import { toast } from "svelte-sonner"
	import { quizzesService } from "$lib/quizzes/service"
	import { attemptsService } from "$lib/attempts/service"
	import { Play, TimerReset } from "lucide-svelte"
	import { useQuery, useMutation, useQueryClient } from "$lib/shared/http/tanstack"

	import Title from "$lib/shared/components/Title.svelte"
	import CertaintyTableView from "$lib/quizzes/components/CertaintyTableView.svelte"

	const joinCode = $derived.by(() => page.url.searchParams.get("joinCode") ?? "")

	const previewQuery = useQuery(() => ({
		queryKey: ["join-preview", joinCode],
		queryFn: () => quizzesService.joinByCode(joinCode),
		enabled: joinCode.length > 0,
	}))

	const hasCertaintyTable = $derived.by(() => {
		if (!previewQuery.data) return false
		return previewQuery.data.isCertaintyBased() && !!previewQuery.data.certaintyTable
	})

	const queryClient = useQueryClient()

	const initializeAttemptMutation = useMutation(() => ({
		mutationFn: () => attemptsService.initialize(previewQuery.data?.id ?? ""),
		onSuccess: async (attempt) => {
			await queryClient.invalidateQueries({ queryKey: ["attempts"] })
			await goto(`/attempts/${attempt.attemptId}?code=${joinCode}`)
		},
		onError: (error) => {
			console.error(error)
			toast.error(error.messageOrDefault)
		},
	}))

	let preAttemptSeconds = $state(0)
	let preAttemptTimerId: ReturnType<typeof setInterval> | null = null

	const startPreAttemptCountdown = (startsAt: string) => {
		if (preAttemptTimerId) clearInterval(preAttemptTimerId)

		const update = () => {
			const diff = Math.max(0, Math.floor((new Date(startsAt).getTime() - Date.now()) / 1000))

			preAttemptSeconds = diff
		}

		update()

		preAttemptTimerId = setInterval(update, 1000)
	}

	$effect(() => {
		if (!previewQuery.data) {
			preAttemptSeconds = 0

			if (preAttemptTimerId) {
				clearInterval(preAttemptTimerId)
				preAttemptTimerId = null
			}

			return
		}

		startPreAttemptCountdown(previewQuery.data.startsAt.value)

		return () => {
			if (preAttemptTimerId) clearInterval(preAttemptTimerId)
		}
	})

	const isPreAttemptWait = $derived(preAttemptSeconds > 0)

	const preAttemptTimerLabel = $derived.by(() => {
		const min = Math.floor(preAttemptSeconds / 60)
		const sec = preAttemptSeconds % 60
		return `${String(min).padStart(2, "0")}:${String(sec).padStart(2, "0")}`
	})
</script>

<section class="flex flex-col gap-4 py-4">
	<Title supra="Unirse a un cuestionario" size="page" tag="h2">
		{previewQuery.data?.title ?? "Sala de espera del cuestionario"}
	</Title>
	<div class="keyline"></div>

	{#if previewQuery.isLoading}
		<p class="m-0 text-zinc-600">Cargando información del cuestionario...</p>
	{:else if previewQuery.isError}
		<p class="m-0 text-red-700">
			{previewQuery.error?.messageOrDefault ?? ""}
		</p>
	{:else if previewQuery.data}
		<div class="grid grid-cols-3 gap-2">
			<div class="stat-card">
				<p class="m-0 text-xs text-zinc-600">Tipo</p>
				<p class="mt-1 mb-0 text-sm font-semibold text-black sm:text-lg">
					{previewQuery.data.kind.toDisplay()}
				</p>
			</div>
			<div class="stat-card">
				<p class="m-0 text-xs text-zinc-600">Preguntas</p>
				<p class="mt-1 mb-0 text-sm font-semibold text-black sm:text-lg">
					{previewQuery.data.questionCount}
				</p>
			</div>
			<div class="stat-card">
				<p class="m-0 text-xs text-zinc-600">Duración</p>
				<p class="mt-1 mb-0 text-sm font-semibold text-black sm:text-lg">
					{previewQuery.data.attemptDurationMinutes} min
				</p>
			</div>
		</div>

		<div class={hasCertaintyTable ? "grid gap-6 lg:grid-cols-2" : "grid"}>
			<div class="flex flex-col gap-1 text-sm leading-relaxed text-zinc-700">
				<p class="m-0 mb-1 font-medium text-black">Antes de comenzar</p>
				<p class="m-0">Inicio programado: {previewQuery.data.startsAt.toDisplay()}</p>
				<p class="m-0">
					Solo tienes un intento. Se entrega automáticamente al agotarse el tiempo.
				</p>
			</div>

			{#if hasCertaintyTable}
				<div class="flex flex-col gap-2">
					<p class="m-0 text-sm font-medium text-black">Tabla de certeza</p>
					<CertaintyTableView table={previewQuery.data.certaintyTable!} />
				</div>
			{/if}
		</div>

		<div class="flex flex-wrap items-center justify-between gap-3">
			<a class="action-tab justify-center" href={resolve("/join")}>
				<TimerReset size={16} /> Cambiar código
			</a>
			<button
				class="btn-primary flex items-center gap-1.5"
				type="button"
				onclick={() => initializeAttemptMutation.mutate()}
				disabled={initializeAttemptMutation.isPending || isPreAttemptWait}
			>
				<Play size={16} aria-hidden="true" />
				{#if initializeAttemptMutation.isPending}
					Iniciando...
				{:else if isPreAttemptWait}
					Comienza en {preAttemptTimerLabel}
				{:else}
					Comenzar intento
				{/if}
			</button>
		</div>
	{/if}
</section>
