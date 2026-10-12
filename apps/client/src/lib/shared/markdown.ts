import { marked, type TokenizerAndRendererExtension, type Tokens } from "marked"
import katex from "katex"
import DOMPurify from "dompurify"

const inlineMath: TokenizerAndRendererExtension = {
	name: "inlineMath",
	level: "inline",
	start(src: string) {
		return src.indexOf("$")
	},
	tokenizer(src: string) {
		const rule = /^\$([^$\n]+)\$/
		const match = rule.exec(src)
		if (match) {
			return {
				type: "inlineMath",
				raw: match[0],
				text: match[1],
			}
		}
	},
	renderer(token: Tokens.Generic) {
		return katex.renderToString(token.text, {
			displayMode: false,
			throwOnError: false,
			output: "html",
		})
	},
}

const blockMath: TokenizerAndRendererExtension = {
	name: "blockMath",
	level: "block",
	start(src: string) {
		return src.indexOf("$$")
	},
	tokenizer(src: string) {
		const rule = /^\$\$([\s\S]+?)\$\$/
		const match = rule.exec(src)
		if (match) {
			return {
				type: "blockMath",
				raw: match[0],
				text: match[1],
			}
		}
	},
	renderer(token: Tokens.Generic) {
		return katex.renderToString(token.text, {
			displayMode: true,
			throwOnError: false,
			output: "html",
		})
	},
}

marked.use({
	gfm: true,
	breaks: false,
	extensions: [inlineMath, blockMath],
})

export const renderMarkdown = (markdown: string): string => {
	const html = marked.parse(markdown, { async: false })
	return DOMPurify.sanitize(html)
}

// Sin bloques (<p>, listas): para textos dentro de botones o de una sola línea.
export const renderMarkdownInline = (markdown: string): string => {
	const html = marked.parseInline(markdown, { async: false })
	return DOMPurify.sanitize(html)
}
