<script lang="ts">
	import type { CertaintyTable } from "../entity"

	interface CertaintyTableViewProps {
		table: CertaintyTable
	}

	let { table }: CertaintyTableViewProps = $props()

	const levels = [
		{ label: "Baja", key: "low" as const },
		{ label: "Media", key: "medium" as const },
		{ label: "Alta", key: "high" as const },
	]
</script>

<div class="panel-surface overflow-x-auto">
	<table class="w-full border-collapse text-sm">
		<thead class="table-head">
			<tr>
				<th class="px-3 py-2 text-left font-medium">Nivel</th>
				<th class="px-3 py-2 text-left font-medium">Correcta</th>
				<th class="px-3 py-2 text-left font-medium">Incorrecta</th>
			</tr>
		</thead>
		<tbody>
			{#each levels as level (level.key)}
				<tr class="border-t border-zinc-200">
					<td class="px-3 py-2 text-zinc-800">{level.label}</td>
					<td class="px-3 py-2 text-zinc-700">{table.get(level.key).correct}</td>
					<td class="px-3 py-2 text-zinc-700">{table.get(level.key).incorrect}</td>
				</tr>
			{/each}
		</tbody>
	</table>
</div>
