<script lang="ts">
	import Markdown from "$lib/shared/components/Markdown.svelte"

	let {
		value = $bindable(""),
		placeholder = "Escribe el enunciado en Markdown...",
	}: { value?: string; placeholder?: string } = $props()

	let mode = $state<"write" | "preview">("write")
</script>

<div class="grid gap-2">
	<div class="segmented">
		<button
			type="button"
			class="segmented-item"
			data-active={mode === "write"}
			onclick={() => (mode = "write")}
		>
			Escribir
		</button>
		<button
			type="button"
			class="segmented-item"
			data-active={mode === "preview"}
			onclick={() => (mode = "preview")}
		>
			Vista previa
		</button>
	</div>

	{#if mode === "write"}
		<textarea class="input-base min-h-32 w-full resize-y font-mono" bind:value {placeholder}
		></textarea>
	{:else}
		<div class="input-base min-h-32 overflow-y-auto">
			{#if value}
				<Markdown content={value} />
			{:else}
				<p class="m-0 text-sm text-zinc-500">Escribe algo para ver la vista previa.</p>
			{/if}
		</div>
	{/if}
</div>
