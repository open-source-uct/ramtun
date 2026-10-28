# AGENTS.md

## Repo Shape
- Not a JS/Rust monorepo: the Rust workspace only contains `apps/server`; the frontend is a standalone pnpm project in `apps/client` with its own lockfile.
- The documentation site is a standalone Astro Starlight project in `apps/docs` with its own lockfile and no backend dependency.
- Backend entrypoint: `apps/server/src/main.rs` — wires modules via `sword::Application`: `auth`, `authz`, `banks`, `courses`, `users`, `quizzes`, `snapshots`, `shared`, `attempts`.
- Frontend shell: `apps/client/src/routes/+layout.svelte` — installs TanStack Query provider and Axios interceptors. SSR is disabled in `+layout.ts`.
- Real-time: Socket.IO server at `/attempts` namespace (events: `attempts:new-submit`, `attempts:new-attempt`, `attempts:warning`). Client in `apps/client/src/lib/shared/socket/attempts.socket.ts`.

## Commands
- Preferred local workflow is Docker-first. `make run` starts the full stack from `compose.yml`; `make detach` runs in background.
- First-time setup: `make setup` (deletes client `node_modules`, runs `corepack pnpm install`, rebuilds containers).
- Repo-wide formatting/lint: `make fmt` / `make lint` (runs both backend and frontend).
- Focused backend from root: `cargo fmt --all`, `cargo clippy --workspace --all-targets --all-features -- -D warnings`, `cargo test -p server`.
- Focused frontend from `apps/client`: `corepack pnpm run lint`, `corepack pnpm run check`, `corepack pnpm run build`.
- Focused docs from `apps/docs`: `pnpm dev`, `pnpm check`, `pnpm build`, `pnpm format`; from root, `make docs`, `make docs-check`, `make docs-build`, `make docs-fmt`.
- CI does not run frontend tests (none exist). Client CI: `lint` + `check`. Server CI: `fmt --check` + `clippy` + `test`.
- DB migration: `make migration name=<name>` (runs `sqlx migrate add`). `make clean-db` drops and recreates the public schema (destructive).
- `make machete` checks for unused Rust dependencies.
- Package management: `make npmi pkg=<name>`, `make npmu pkg=<name>`, `make npmci`.

## Docker And Env
- Root `.env` is loaded by `make`, `compose.yml`, and Vite (`apps/client/vite.config.ts` uses `envDir: "../../"`). There is no `apps/client/.env`.
- Local entrypoint: nginx container on port `80`. Nginx proxies `/` → client:5173, `/api/` → server:8000.
- Frontend Axios `baseURL: "/api"` — preserve this proxy path unless intentionally changing nginx wiring.
- Server runs SQL migrations automatically in `Database::new()` from `apps/server/config/migrations`.
- Server dev container entrypoint (`apps/server/config/scripts/entrypoint.sh`) runs `cargo watch` only — no VPN/WireGuard setup locally.
- Compose services: `server`, `nginx`, `postgres`, `client`, `ldap`, `ldap-ui`. LDAP services are for auth integration; `ldap-ui` is on port `8080`.

## Code Navigation
- Backend features: `apps/server/src/<feature>/` with `controller`, `service`, `repository`, and optional `policy`/`dtos`/`entity`. Start there.
- Frontend features: `apps/client/src/lib/features/*` (attempts, banks, courses, quizzes, users). Route files are thin wrappers.
- Auth/session: `apps/client/src/lib/auth/service.ts` and `apps/client/src/lib/shared/http/api.interceptors.ts`. Protected-route redirect: `apps/client/src/routes/(protected)/+layout.ts`.
- Anti-cheat system: `apps/client/src/lib/attempts/services/anti-cheat.service.svelte.ts` — blocks context menu, copy, search, screenshot, alt-tab. Warnings broadcast via Socket.IO.
- API docs: `.docs/endpoints/` contains per-resource markdown docs (auth, users, courses, banks, quizzes, attempts).
- Documentation site: content in `apps/docs/src/content/docs/`, diagram gallery data in `apps/docs/src/data/diagrams.ts`.

## Style Notes
- Frontend formatting from root `.prettierrc`: tabs, no semicolons, `printWidth: 100`, double quotes, Tailwind class sorting via `prettier-plugin-tailwindcss` using `apps/client/src/routes/layout.css`.
- Frontend uses Svelte 5 runes (`$state`, `$derived`) — not legacy store patterns. Match existing style.
- Schema validation: `valibot` (not zod or yup).
- Key frontend deps: `@tanstack/svelte-query` v6, `axios`, `socket.io-client`, `marked`+`dompurify` (markdown), `katex` (math), `lucide-svelte` (icons), `svelte-sonner` (toasts), `@formisch/svelte` (forms), `runed` (runes utilities).

## Frontend Copy Rules
- **No implementation leaks.** Visible text must never reference internal architecture, contracts, endpoints, policies, authorization roles, technical jargon, or development status.
- **User-oriented copy only.** Describe what the user can do, not how the system works.
- **No generic kickers without context.** Avoid meaningless section labels like "Flujo estudiante", "Gestion academica", "Curso", or "Quiz".
- **Consistent language.** Prefer "Crea y administra quizzes de este curso." over "Gestion de quizzes por curso: creacion, listado y acciones administrativas."
- **Error messages** should be actionable and in Spanish, but may reference HTTP status codes when helpful.

## Deployment
- Production images built via `ghcr-images.yml` CI workflow → GHCR (`ramtun-server`, `ramtun-client`).
- K8s deployer: `make deployer cmd=<deploy|db|sh|kubectl> [args=...]` — builds and runs a container with kubectl + WireGuard. Uses `.env.prod` and `~/.kube`.
- K8s manifests in `config/deployer/k8s/`: server, client, postgres (CloudNativePG), ingress at `ramtun.inf.uct.cl`.
- `compose.prod.yml` is a simplified production compose (no nginx — client Dockerfile prod stage includes it).
