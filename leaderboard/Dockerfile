# 1) Install dependencies with Bun
FROM oven/bun:latest AS deps
WORKDIR /app

# Copy lockfile(s) & manifest to leverage caching
COPY bun.lock package.json ./
RUN bun install

# 2) Build the SvelteKit app
FROM oven/bun:latest AS builder
WORKDIR /app

# Reuse installed deps and copy source
COPY --from=deps /app/node_modules ./node_modules
COPY . .

# Build (uses your adapter – e.g. @sveltejs/adapter-node or adapter-static)
RUN bun run build

# 3) Production image
FROM oven/bun:latest AS runner
WORKDIR /app

# Only bring in what's needed at runtime
COPY --from=builder /app/build ./build
COPY --from=builder /app/node_modules ./node_modules
COPY package.json ./

# Ensure production mode
ENV NODE_ENV=production

# Expose default SvelteKit port
EXPOSE 3000

# Start via the "start" script in your package.json
CMD ["bun", "run", "start"]
