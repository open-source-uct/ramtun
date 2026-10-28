<script lang="ts">
	import { Moon, Sun } from "lucide-svelte"
	import { themeStore } from "$lib/shared/theme.svelte"

	const isDark = $derived(themeStore.current === "dark")
</script>

<button
	class="btn-close size-11 cursor-pointer"
	type="button"
	onclick={themeStore.toggle}
	title={isDark ? "Cambiar a modo claro" : "Cambiar a modo oscuro"}
	aria-label={isDark ? "Cambiar a modo claro" : "Cambiar a modo oscuro"}
>
	{#if isDark}
		<Sun size={16} aria-hidden="true" />
	{:else}
		<Moon size={16} aria-hidden="true" />
	{/if}
</button>
