<script lang="ts">
	import type { LayoutProps } from "./$types"

	import { goto } from "$app/navigation"
	import { page } from "$app/state"
	import { toast } from "svelte-sonner"
	import { onMount } from "svelte"
	import { authStore } from "$lib/auth/store.svelte"
	import { useMutation, useQuery } from "$lib/shared/http/tanstack"
	import { authService } from "$lib/auth/service"
	import { DoorOpen, Layers, ClipboardList, LogOut } from "lucide-svelte"
	import {
		connectAttemptsSocket,
		disconnectAttemptsSocket,
	} from "$lib/shared/socket/attempts.socket"
	import { attemptsService } from "$lib/attempts/service"

	let { data, children }: LayoutProps = $props()

	const user = authStore.user
	const showCoursesNav = $derived(user?.isAdmin() || user?.isFunc() || data.hasCourses)

	$effect(() => {
		if (user?.isStudent() && !data.hasCourses && page.url.pathname.startsWith("/courses")) {
			void goto("/join")
		}
	})

	const activeAttemptQuery = useQuery(() => ({
		queryKey: ["attempts", "active"] as const,
		queryFn: () => attemptsService.getActive(),
		retry: false,
		refetchOnMount: false,
		refetchOnWindowFocus: false,
	}))

	$effect(() => {
		const active = activeAttemptQuery.data
		if (!active) return

		const attemptPath = `/attempts/${active.attemptId}`

		if (!page.url.pathname.startsWith(attemptPath)) {
			void goto(attemptPath)
		}
	})

	const logoutMutation = useMutation(() => ({
		mutationFn: () => authService.logout(),
		onSettled: async () => {
			authStore.clearSession()
			toast.info("Cerrando sesión...")
			await goto("/login")
		},
	}))

	const handleLogout = async () => {
		await logoutMutation.mutateAsync()
	}

	const isActive = (href: string) => {
		return page.url.pathname.startsWith(href)
	}

	onMount(() => {
		if (authStore.isAuthenticated()) {
			connectAttemptsSocket()
		}

		return () => disconnectAttemptsSocket()
	})
</script>

<main class="mx-auto flex min-h-dvh max-w-312 flex-col gap-2 p-3 sm:p-4 lg:p-6">
	<header
		class="panel-elevated mx-auto flex w-full items-center justify-between gap-2 px-2 py-2 sm:grid sm:grid-cols-[1fr_auto_1fr] sm:gap-4 sm:px-8 sm:py-3.5 lg:gap-8"
	>
		<div class="min-w-0">
			<p class="m-0 text-xs font-semibold tracking-widest text-zinc-500">INF-UCT: RAMTUN</p>
			<h1 class="mt-0.5 mb-0 text-base leading-tight text-black sm:text-2xl">
				Cuestionarios en línea
			</h1>
		</div>

		<nav class="flex items-center sm:gap-2 lg:gap-6" aria-label="Principal">
			<a
				class="text-tab min-h-11 min-w-11 justify-center lg:min-w-0"
				data-active={isActive("/join")}
				href="/join"
				title="Unirse"
			>
				<DoorOpen size={16} aria-hidden="true" />
				<span class="hidden lg:inline">Unirse</span>
			</a>
			<a
				class="text-tab min-h-11 min-w-11 justify-center lg:min-w-0"
				data-active={isActive("/results")}
				href="/results"
				title="Resultados"
			>
				<ClipboardList size={16} aria-hidden="true" />
				<span class="hidden lg:inline">Resultados</span>
			</a>
			{#if showCoursesNav}
				<a
					class="text-tab min-h-11 min-w-11 justify-center lg:min-w-0"
					data-active={isActive("/courses")}
					href="/courses"
					title="Cursos"
				>
					<Layers size={16} aria-hidden="true" />
					<span class="hidden lg:inline">Cursos</span>
				</a>
			{/if}
		</nav>

		<div class="flex min-w-0 items-center gap-2.5 justify-self-end">
			<div class="hidden min-w-0 leading-tight sm:block">
				<p class="m-0 max-w-56 truncate text-sm font-semibold text-zinc-800">
					{authStore.user?.name}
				</p>
				<p class="m-0 text-xs text-zinc-600">{authStore.user?.role.toDisplay()}</p>
			</div>
			<button
				class="btn-close size-11 cursor-pointer"
				type="button"
				onclick={handleLogout}
				title="Cerrar sesión"
			>
				<LogOut size={16} aria-hidden="true" />
			</button>
		</div>
	</header>

	<section class="panel-elevated mx-auto w-full min-w-0 px-3 py-4 sm:px-12">
		{@render children()}
	</section>
</main>
