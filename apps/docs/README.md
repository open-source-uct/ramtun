# Ramtun Docs

Sitio de documentación interna construido con Astro Starlight. El contenido canónico está en
español; los identificadores y nombres de código se mantienen en inglés.

## Comandos

```sh
pnpm install
pnpm dev
pnpm check
pnpm build
pnpm preview
```

El build ejecuta `astro check` antes de generar el sitio.

## Secciones

1. **Visión general** — qué es Ramtun, alcance y glosario.
2. **Empezar** — instalación y configuración, flujo de trabajo y problemas frecuentes.
3. **Modelo de dominio** — entidades, roles y ciclo de vida del intento.
4. **Base de datos y migraciones** — esquema, migraciones y soft delete.
5. **Formato de bancos** — estructura, sintaxis y validación de preguntas.
6. **Calificación por certeza** — puntaje y criterios de aprobación.
7. **Arquitectura** — límites, flujos y decisiones técnicas.
8. **Backend** — módulos del servidor.
9. **Frontend** — aplicación SvelteKit, estado y experiencia de usuario.
10. **API** — contratos HTTP y eventos en tiempo real.
11. **Infraestructura** — contenedores, despliegue, seguridad y observabilidad.
12. **Contribuir** — estructura del repositorio, documentación y guías de cambio.
13. **Diagramas** — galería visual de referencia.

## Estructura

- `src/content/docs/` — capítulos y páginas. Cada archivo declara `title`, `description`, `owner`,
  `status` y `audience` en el frontmatter.
- `src/data/diagrams.ts` — índice de diagramas de la galería.
- `public/diagrams/` — imágenes de los diagramas.
- `src/styles/` — tema visual y estilos de componentes.
- `templates/` — plantillas para nuevas páginas.

## Añadir un diagrama

1. Guarda la imagen en `public/diagrams/`.
2. Agrega una entrada en `src/data/diagrams.ts` con `title`, `description`, `href`, `kind` e
   `image`.
3. Enlaza `href` a la página donde se explica el diagrama.
