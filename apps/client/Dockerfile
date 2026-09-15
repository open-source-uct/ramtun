# BASE Stage =====================================

FROM node:24-alpine AS base
WORKDIR /app

RUN apk add --no-cache curl

COPY ./apps/client/package.json ./
COPY ./apps/client/pnpm-lock.yaml ./
COPY ./apps/client/.npmrc ./

RUN corepack enable && pnpm install --frozen-lockfile --ignore-scripts

# BUILD Stage =====================================

FROM base AS builder
COPY ./apps/client/ ./
RUN pnpm build

# PRODUCTION Stage =====================================

FROM nginx:alpine AS prod

COPY --from=builder /app/build /usr/share/nginx/html
COPY config/nginx.prod.conf.template /etc/nginx/templates/default.conf.template
COPY config/nginx-security-headers.conf /etc/nginx/snippets/security-headers.conf

ENV API_URL=http://ramtun-server:8000
ENV NGINX_ENVSUBST_FILTER=^API_

EXPOSE 80

CMD ["nginx", "-g", "daemon off;"]

# DEVELOPMENT Stage =====================================

FROM base AS dev
ARG UID=1000
ARG GID=1000

RUN chown -R ${UID}:${GID} /app
USER ${UID}:${GID}

EXPOSE 5173

CMD ["pnpm", "dev", "--host", "0.0.0.0", "--port", "5173"]
