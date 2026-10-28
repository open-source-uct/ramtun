import { FileText, Library, Users } from "lucide-svelte"

/** Secciones de un curso: fuente única de las pestañas del curso y del menú del breadcrumb. */
export const courseSections = (courseId: string) => {
	const base = `/courses/${courseId}`

	return [
		{
			label: "Cuestionarios",
			icon: FileText,
			href: `${base}/quizzes`,
			isActive: (pathname: string) =>
				pathname === base || pathname.startsWith(`${base}/quizzes`),
		},
		{
			label: "Bancos de preguntas",
			icon: Library,
			href: `${base}/banks`,
			isActive: (pathname: string) => pathname.startsWith(`${base}/banks`),
		},
		{
			label: "Miembros del curso",
			icon: Users,
			href: `${base}/members`,
			isActive: (pathname: string) => pathname.startsWith(`${base}/members`),
		},
	]
}
