
export interface Diagram {
	title: string
	description: string
	href: string
	image?: string
	width?: number
	height?: number
}

export const diagrams: Diagram[] = [
	{
		title: "Modelo de datos",
		description: "Entidades, relaciones y restricciones principales del dominio de Ramtun.",
		href: "/dominio/entidades/",
		image: "/diagrams/ramtun-erd.png",
		width: 2692,
		height: 1347,
	},
]
