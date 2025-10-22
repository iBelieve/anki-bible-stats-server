# Stage 1: Builder - Install dependencies and build the app
FROM node:22-alpine AS builder

WORKDIR /app

# Copy package files
COPY package*.json ./

# Install all dependencies (including dev dependencies needed for build)
RUN npm ci

# Copy source code
COPY . .

# Build the SvelteKit app for production
RUN npm run build

# Prune dev dependencies for smaller runtime image
RUN npm prune --production

# Stage 2: Runtime - Minimal image with just the built app
FROM node:22-alpine AS runtime

WORKDIR /app

# Install runtime dependencies only
RUN apk add --no-cache \
    ca-certificates

# Copy production dependencies from builder
COPY --from=builder --chown=node:node /app/node_modules ./node_modules

# Copy built app from builder
COPY --from=builder --chown=node:node /app/build ./build
COPY --from=builder --chown=node:node /app/package.json ./

USER node

# Expose frontend port
EXPOSE 3000

# Set Node environment to production
ENV NODE_ENV=production

# Run the Node.js server
CMD ["node", "build"]
