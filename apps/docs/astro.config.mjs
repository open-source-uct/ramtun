import { defineConfig } from "astro/config"
import starlight from "@astrojs/starlight"
import starlightLinksValidator from "starlight-links-validator"

const repository = "https://github.com/open-source-uct/ramtun"

export default defineConfig({
	site: "https://docs.ramtun.inf.uct.cl",
	trailingSlash: "always",
	integrations: [
		starlight({
			title: "Documentación",
			description: "Documentación interna para comprender, construir y operar Ramtun.",
			logo: {
				light: "./src/assets/logo-light.svg",
				dark: "./src/assets/logo-dark.svg",
				alt: "Ramtun",
			},
			favicon: "/favicon.svg",
			locales: {
				root: {
					label: "Español",
					lang: "es",
				},
			},
			social: [
				{
					icon: "github",
					label: "GitHub",
					href: repository,
				},
			],
			editLink: {
				baseUrl: `${repository}/edit/main/apps/docs/`,
			},
			lastUpdated: true,
			pagination: true,
			disable404Route: true,
			customCss: [
				"@fontsource-variable/jetbrains-mono",
				"./src/styles/theme.css",
				"./src/styles/components.css",
			],
			head: [
				{
					tag: "meta",
					attrs: {
						name: "robots",
						content: "noindex, nofollow",
					},
				},
			],
			sidebar: [
				{
					label: "Inicio",
					items: [{ label: "Portada", link: "/" }],
				},
				{
					label: "Visión general",
					items: [{ autogenerate: { directory: "vision-general" } }],
				},
				{
					label: "Empezar",
					items: [{ autogenerate: { directory: "empezar" } }],
				},
				{
					label: "Modelo de dominio",
					items: [{ autogenerate: { directory: "dominio" } }],
				},
				{
					label: "Base de datos y migraciones",
					items: [{ autogenerate: { directory: "base-de-datos" } }],
				},
				{
					label: "Formato de bancos",
					items: [{ autogenerate: { directory: "formato-de-bancos" } }],
				},
				{
					label: "Calificación por certeza",
					items: [{ autogenerate: { directory: "calificacion" } }],
				},
				{
					label: "Arquitectura",
					items: [{ autogenerate: { directory: "arquitectura" } }],
				},
				{
					label: "Backend",
					items: [{ autogenerate: { directory: "backend" } }],
				},
				{
					label: "Frontend",
					items: [{ autogenerate: { directory: "frontend" } }],
				},
				{
					label: "API",
					items: [{ autogenerate: { directory: "api" } }],
				},
				{
					label: "Infraestructura",
					collapsed: true,
					items: [{ autogenerate: { directory: "infraestructura" } }],
				},
				{
					label: "Contribuir",
					collapsed: true,
					items: [{ autogenerate: { directory: "contribuir" } }],
				},
				{
					label: "Diagramas",
					items: [{ autogenerate: { directory: "diagramas" } }],
				},
			],
			plugins: [starlightLinksValidator()],
		}),
	],
})
