<script lang="ts">
	import { Check, ChevronRight } from "lucide-svelte"

	interface BreadcrumbMenuProps {
		label: string
		items: { label: string; href: string; current?: boolean }[]
		empty?: string
		disabled?: boolean
	}

	let { label, items, empty = "Sin elementos", disabled = false }: BreadcrumbMenuProps = $props()

	let open = $state(false)
	let root = $state<HTMLElement>()

	$effect(() => {
		if (disabled) open = false
	})
</script>

<svelte:window
	onpointerdown={(event) => {
		if (open && root && !root.contains(event.target as Node)) open = false
	}}
	onkeydown={(event) => {
		if (open && event.key === "Escape") open = false
	}}
/>

<!-- El separador del breadcrumb es el disparador: abre las alternativas de ese nivel. -->
<div class="relative mx-1 shrink-0" bind:this={root}>
	<button
		class="flex size-6 cursor-pointer items-center justify-center rounded-[4px] text-zinc-500 hover:bg-zinc-200 hover:text-black disabled:cursor-default disabled:hover:bg-transparent disabled:hover:text-zinc-500"
		type="button"
		aria-label={label}
		aria-expanded={open}
		{disabled}
		onclick={() => (open = !open)}
	>
		<ChevronRight
			class="transition-transform duration-150 {open ? 'rotate-90' : ''}"
			size={14}
			aria-hidden="true"
		/>
	</button>

	{#if open}
		<ul
			class="panel-elevated absolute top-full left-0 z-20 m-0 mt-1.5 max-h-72 w-72 max-w-[80vw] list-none overflow-y-auto p-1"
			aria-label={label}
		>
			{#each items as item (item.href)}
				<li>
					<a
						class="row-hover flex items-center justify-between gap-2 rounded-[4px] px-2.5 py-2 text-sm text-zinc-800 hover:text-black"
						class:font-medium={item.current}
						href={item.href}
						aria-current={item.current ? "page" : undefined}
						onclick={() => (open = false)}
					>
						<span class="truncate">{item.label}</span>
						{#if item.current}
							<Check class="shrink-0" size={14} aria-hidden="true" />
						{/if}
					</a>
				</li>
			{:else}
				<li class="px-2.5 py-2 text-sm text-zinc-600">{empty}</li>
			{/each}
		</ul>
	{/if}
</div>
