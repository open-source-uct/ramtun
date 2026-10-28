import { defineCollection } from "astro:content"
import { z } from "astro/zod"
import { docsLoader, i18nLoader } from "@astrojs/starlight/loaders"
import { docsSchema, i18nSchema } from "@astrojs/starlight/schema"

export const collections = {
	docs: defineCollection({
		loader: docsLoader(),
		schema: docsSchema({
			extend: z.object({
				description: z.string().min(40).max(160),
				owner: z.string().min(2).startsWith("@"),
				status: z.enum(["draft", "review", "stable", "deprecated"]).default("draft"),
				reviewedAt: z.coerce.date().optional(),
				audience: z.enum(["dev", "ops", "api", "all"]).default("dev"),
			}),
		}),
	}),
	i18n: defineCollection({
		loader: i18nLoader(),
		schema: i18nSchema(),
	}),
}
