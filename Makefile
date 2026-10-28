ifneq (,$(wildcard ./.env))
    include .env
    export
endif

RUST_VERSION := $(shell grep 'channel' rust-toolchain.toml | head -1 | sed 's/.*"//;s/"//')
export RUST_VERSION

SERVER=apps/server
CLIENT=apps/client
DOCS=apps/docs
PNPM ?= pnpm
app ?= "server"
pkg ?=
service ?= $(app)
file ?=
cmd ?= deploy
args ?=
docker_tty ?= -it

.DEFAULT_GOAL := run
.PHONY: run detach clean fmt lint migration machete clean-db db npmi npmu npmci setup deployer down spell docs docs-check docs-build docs-fmt

setup:
	rm -rf $(CLIENT)/node_modules
	cd $(CLIENT) && corepack pnpm install --frozen-lockfile --ignore-scripts
	docker compose build
	docker compose up client -d --force-recreate --renew-anon-volumes

run:
	docker compose up

down:
	docker compose down

detach:
	docker compose up -d

clean:
	docker compose down -v

clean-db:
	docker exec ramtun-postgres psql -U user -d database -c "DROP SCHEMA public CASCADE; CREATE SCHEMA public;"
	cd $(SERVER) && sqlx migrate run --source ./config/migrations --database-url $(LOCAL_POSTGRES_DATABASE_URL)

db:
	docker exec -it ramtun-postgres /bin/bash -c "psql -U user -d ramtun"

fmt:
	cargo fmt --all
	cd $(CLIENT) && corepack pnpm run format

lint:
	cargo clippy --workspace --all-targets --all-features -- -D warnings
	cd $(CLIENT) && corepack pnpm run lint
	cd $(CLIENT) && corepack pnpm run check

spell:
	corepack pnpm --package=cspell --package=@cspell/dict-es-es dlx cspell lint --no-progress "$(CLIENT)/src/**/*.{svelte,ts,css}"

migration:
	cd $(SERVER) && sqlx migrate add --source ./config/migrations "$(name)"

machete:
	cargo machete

npmi:
	cd apps/$(app) && corepack pnpm add $(pkg)
	docker compose exec $(service) /bin/sh -c "cd /app/apps/$(app) && corepack pnpm add $(pkg)"

npmu:
	cd $(CLIENT) && corepack pnpm remove $(pkg)
	docker compose exec $(service) /bin/sh -c "cd /app/apps/$(app) && corepack pnpm remove $(pkg)"

npmci:
	cd $(CLIENT) && corepack pnpm install --frozen-lockfile --ignore-scripts
	docker compose exec $(service) /bin/sh -c "cd /app/apps/$(app) && corepack pnpm install --frozen-lockfile --ignore-scripts"

deployer:
	docker build -f config/deployer/Dockerfile -t questions-deployer:local .
	docker run --rm $(docker_tty) --cap-add=NET_ADMIN --device=/dev/net/tun --env-file .env.prod -v "$(PWD)":/workspace -v "$$HOME/.kube":/root/.kube:ro questions-deployer:local $(cmd) $(args)

seed:
	cargo run -p server --bin seeder

docs:
	cd $(DOCS) && $(PNPM) install && $(PNPM) dev

docs-check:
	cd $(DOCS) && $(PNPM) check

docs-build:
	cd $(DOCS) && $(PNPM) build

docs-fmt:
	cd $(DOCS) && $(PNPM) format
