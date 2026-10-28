<script lang="ts">
	import { onMount } from "svelte"
	import { Toaster } from "svelte-sonner"
	import { queryClient } from "$lib/shared/http/tanstack"
	import { themeStore } from "$lib/shared/theme.svelte"
	import { QueryClientProvider } from "@tanstack/svelte-query"
	import "katex/dist/katex.min.css"

	import "./layout.css"

	let { children } = $props()

	onMount(themeStore.init)
</script>

<svelte:head>
	<title>INF UCT - Ramtun</title>
</svelte:head>

<QueryClientProvider client={queryClient}>
	<Toaster richColors position="top-right" theme={themeStore.current} />
	{@render children()}
</QueryClientProvider>
