<script lang="ts">
	import { page } from "$app/state"
	import { courseSections } from "$lib/courses/sections"
	import Title from "$lib/shared/components/Title.svelte"

	let { children, data } = $props()

	const sections = $derived(courseSections(data.courseId))

	// El detalle de un banco muestra su breadcrumb en lugar del título y las pestañas del curso.
	const isBankDetail = $derived(page.params.bankId !== undefined)
</script>

<section class="flex flex-col py-2">
	{#if !isBankDetail}
		<header class="mb-5">
			<div class="flex flex-wrap items-baseline gap-x-3 gap-y-1">
				<Title size="page">{data.course.name}</Title>
				<p class="m-0 text-sm text-zinc-600">{data.course.code} · {data.course.year}</p>
			</div>

			<nav
				class="mt-3 flex gap-6 overflow-x-auto border-b border-zinc-300"
				aria-label="Secciones del curso"
			>
				{#each sections as { label, icon: Icon, href, isActive } (href)}
					<a class="text-tab" data-active={isActive(page.url.pathname)} {href}>
						<Icon size={16} aria-hidden="true" />
						{label}
					</a>
				{/each}
			</nav>
		</header>
	{/if}

	<div>
		{#key page.url.pathname}
			{@render children()}
		{/key}
	</div>
</section>
