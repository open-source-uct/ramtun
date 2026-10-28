<script lang="ts">
	interface QuizOptionProps {
		text: string
		isSelected: boolean
		onclick: () => void
	}

	import Markdown from "$lib/shared/components/Markdown.svelte"

	let { text, isSelected, onclick }: QuizOptionProps = $props()
</script>

<button type="button" class="quiz-option" data-active={isSelected} {onclick}>
	<Markdown content={text} inline />
</button>
