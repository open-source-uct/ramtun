<script lang="ts">
	import { renderMarkdown, renderMarkdownInline } from "$lib/shared/markdown"

	// `inline` produce HTML sin bloques dentro de un span raíz, válido en button o label.
	let { content, inline = false }: { content: string; inline?: boolean } = $props()

	const html = $derived(inline ? renderMarkdownInline(content) : renderMarkdown(content))
</script>

<svelte:element this={inline ? "span" : "div"} class="markdown-body" class:markdown-inline={inline}>
	<!-- eslint-disable-next-line svelte/no-at-html-tags -- HTML is sanitized with DOMPurify before rendering -->
	{@html html}
</svelte:element>

<style>
	.markdown-inline {
		display: block;
		min-width: 0;
		overflow-wrap: break-word;
	}

	.markdown-body :global(p) {
		margin: 0 0 0.75em;
	}

	.markdown-body :global(p:last-child) {
		margin-bottom: 0;
	}

	.markdown-body :global(h1),
	.markdown-body :global(h2),
	.markdown-body :global(h3),
	.markdown-body :global(h4) {
		margin: 0 0 0.5em;
		font-weight: 600;
		color: #171412;
	}

	.markdown-body :global(h1) {
		font-size: 1.5rem;
	}

	.markdown-body :global(h2) {
		font-size: 1.25rem;
	}

	.markdown-body :global(h3) {
		font-size: 1.125rem;
	}

	.markdown-body :global(h4) {
		font-size: 1rem;
	}

	.markdown-body :global(ul),
	.markdown-body :global(ol) {
		margin: 0 0 0.75em;
		padding-left: 1.5rem;
	}

	.markdown-body :global(li) {
		margin: 0.25em 0;
	}

	.markdown-body :global(a) {
		color: #1c6c48;
		text-decoration: underline;
	}

	.markdown-body :global(blockquote) {
		margin: 0 0 0.75em;
		border-left: 3px solid #d7d0c8;
		padding-left: 1rem;
		color: #58514a;
	}

	.markdown-body :global(hr) {
		margin: 1em 0;
		border: 0;
		border-top: 1px solid #d7d0c8;
	}

	.markdown-body :global(code) {
		border-radius: 4px;
		background: #f0f0eb;
		padding: 0.125em 0.375em;
		font-family: ui-monospace, SFMono-Regular, Menlo, Consolas, monospace;
		font-size: 0.875em;
	}

	.markdown-body :global(pre) {
		margin: 0 0 0.75em;
		overflow-x: auto;
		border-radius: 6px;
		background: #fafaf8;
		padding: 0.875rem 1rem;
	}

	.markdown-body :global(pre code) {
		background: transparent;
		padding: 0;
		font-size: 0.875rem;
	}

	.markdown-body :global(.katex-display) {
		overflow-x: auto;
		overflow-y: hidden;
	}

	.markdown-body :global(table) {
		margin: 0 0 0.75em;
		border-collapse: collapse;
	}

	.markdown-body :global(th),
	.markdown-body :global(td) {
		border: 1px solid #d7d0c8;
		padding: 0.375rem 0.625rem;
	}

	.markdown-body :global(th) {
		background: #f0f0eb;
		font-weight: 600;
	}

	.markdown-body :global(img) {
		max-width: 100%;
		border-radius: 4px;
	}
</style>
