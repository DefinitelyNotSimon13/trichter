FROM oven/bun:latest AS deps
WORKDIR /app

COPY bun.lock package.json ./
RUN bun install

FROM oven/bun:latest AS builder
WORKDIR /app
COPY --from=deps /app/node_modules ./node_modules

COPY . .
RUN bun run build


FROM oven/bun:latest AS runner
WORKDIR /app

COPY --from=builder /app/build ./build
COPY --from=builder /app/node_modules ./node_modules
COPY package.json ./

ENV NODE_ENV=production

EXPOSE 3000

CMD ["bun", "run", "start"]
