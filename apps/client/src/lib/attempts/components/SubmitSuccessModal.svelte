<script lang="ts">
	import { fade } from "svelte/transition"
	import { CircleCheck } from "lucide-svelte"

	interface SubmitSuccessModalProps {
		open: boolean
		onContinue: () => void
	}

	let { open, onContinue }: SubmitSuccessModalProps = $props()
</script>

{#if open}
	<div
		class="fixed inset-0 z-50 flex items-center justify-center bg-(--scrim) p-4"
		transition:fade={{ duration: 150 }}
		role="dialog"
		aria-modal="true"
		tabindex="-1"
	>
		<div class="panel-surface w-full max-w-md p-6" role="presentation" tabindex="-1">
			<div class="flex flex-col items-center gap-4 py-6">
				<CircleCheck size={32} class="text-emerald-600" aria-hidden="true" />
				<h3 class="m-0 text-xl text-black">Intento enviado</h3>
				<p class="m-0 text-center text-sm text-zinc-600">
					Tu intento fue enviado correctamente. Los resultados estarán disponibles cuando
					el docente los publique.
				</p>
				<button class="btn-primary mt-2" type="button" onclick={onContinue}>
					Ir a resultados
				</button>
			</div>
		</div>
	</div>
{/if}
