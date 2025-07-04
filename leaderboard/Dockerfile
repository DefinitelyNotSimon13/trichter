FROM oven/bun:latest AS builder
WORKDIR /app

COPY bun.lock package.json ./
RUN bun install

COPY . .

RUN bun run build

FROM node:24-alpine AS runner
WORKDIR /app
ENV NODE_ENV=production

COPY --from=builder /app/build ./build
COPY --from=builder /app/node_modules ./node_modules
COPY package.json ./


EXPOSE 3000

CMD ["node", "build"]

LABEL org.opencontainers.image.source=https://github.com/DefinitelyNotSimon13/trichter
