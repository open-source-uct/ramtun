<script lang="ts">
	import type { AttemptResult } from "$lib/attempts/dtos"

	import { goto } from "$app/navigation"
	import { page } from "$app/state"
	import { onMount } from "svelte"

	import AttemptResultReview from "$lib/attempts/components/AttemptResultReview.svelte"

	let result = (page.state as { results?: AttemptResult })?.results

	// A reload loses page.state: fetch again by code, which the server allows only for quizzes with multiple views.
	onMount(async () => {
		if (result) return
		const joinCode = page.url.searchParams.get("joinCode")
		await goto(joinCode ? `/results?code=${encodeURIComponent(joinCode)}` : "/results", {
			replaceState: true,
		})
	})
</script>

{#if result}
	<AttemptResultReview {result} />
{/if}
