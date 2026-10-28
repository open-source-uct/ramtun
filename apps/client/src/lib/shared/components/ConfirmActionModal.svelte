<script lang="ts">
	import { fade, scale } from "svelte/transition"

	interface ConfirmActionModalProps {
		open: boolean
		title: string
		message: string
		confirmLabel?: string
		cancelLabel?: string
		isPending?: boolean
		onConfirm: () => void | Promise<void>
		onCancel: () => void
	}

	let {
		open,
		title,
		message,
		confirmLabel = "Confirmar",
		cancelLabel = "Cancelar",
		isPending = false,
		onConfirm,
		onCancel,
	}: ConfirmActionModalProps = $props()
</script>

{#if open}
	<div
		class="fixed inset-0 z-50 flex items-center justify-center bg-(--scrim) p-4"
		role="dialog"
		aria-modal="true"
		tabindex="-1"
		transition:fade={{ duration: 180 }}
		onclick={onCancel}
		onkeydown={(e) => {
			if (e.key === "Escape" && !isPending) onCancel()
		}}
	>
		<section
			class="panel-elevated w-full max-w-md p-5"
			role="presentation"
			tabindex="-1"
			transition:scale={{ duration: 190, start: 0.98 }}
			onclick={(e) => e.stopPropagation()}
		>
			<h4 class="m-0 text-base text-black">{title}</h4>
			<p class="mt-2 mb-0 text-sm text-zinc-700">{message}</p>
			<div class="mt-4 flex justify-end gap-2">
				<button class="btn-tertiary" type="button" onclick={onCancel} disabled={isPending}>
					{cancelLabel}
				</button>
				<button class="btn-primary" type="button" onclick={onConfirm} disabled={isPending}>
					{isPending ? "Procesando..." : confirmLabel}
				</button>
			</div>
		</section>
	</div>
{/if}
