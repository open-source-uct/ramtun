export type Theme = "light" | "dark"

const STORAGE_KEY = "theme"
const darkQuery = () => window.matchMedia("(prefers-color-scheme: dark)")

// Fuente de verdad de la clase `dark` en <html>. El script de `app.html` la fija antes del primer
// pintado; este store solo la lee, la alterna y la mantiene sincronizada con el sistema mientras
// la persona no haya elegido un tema a mano.
class ThemeStore {
	current = $state<Theme>("light")

	init = () => {
		this.current = document.documentElement.classList.contains("dark") ? "dark" : "light"

		const query = darkQuery()
		const followSystem = () => {
			if (localStorage.getItem(STORAGE_KEY)) return
			this.apply(query.matches ? "dark" : "light")
		}
		query.addEventListener("change", followSystem)
		return () => query.removeEventListener("change", followSystem)
	}

	toggle = () => {
		const next = this.current === "dark" ? "light" : "dark"
		localStorage.setItem(STORAGE_KEY, next)
		this.apply(next)
	}

	private readonly apply = (theme: Theme) => {
		document.documentElement.classList.toggle("dark", theme === "dark")
		this.current = theme
	}
}

export const themeStore = new ThemeStore()
