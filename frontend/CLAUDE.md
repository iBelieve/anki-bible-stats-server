# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is a SvelteKit application configured to deploy as a Node.js application using the Node adapter. The project uses:
- **SvelteKit** with Svelte 5 (using new runes syntax like `$props()`)
- **TypeScript** for type safety
- **Tailwind CSS v4** (via Vite plugin) for styling
- **Node.js** as the deployment target (containerized via Docker)

## Development Commands

```bash
# Start development server
npm run dev

# Type checking
npm run check              # Run once
npm run check:watch        # Watch mode

# Code quality
npm run lint              # Check formatting and linting
npm run format            # Auto-format with Prettier

# Build
npm run build            # Production build for Node.js
npm run preview          # Preview production build locally
```

## Project Structure

- `src/routes/` - SvelteKit file-based routing
  - `+page.svelte` - Page components
  - `+layout.svelte` - Layout components (wraps pages)
- `src/lib/` - Reusable components and utilities (accessible via `$lib` alias)
- `src/app.html` - HTML template
- `src/app.css` - Global styles (Tailwind imports)
- `static/` - Static assets served from root

## Architecture Notes

**Svelte 5 Syntax**: This project uses Svelte 5 with runes. Use `$props()`, `$state()`, `$derived()` instead of legacy syntax.

**Styling**: Tailwind CSS v4 is configured through the Vite plugin (`@tailwindcss/vite`). Import Tailwind directives in `src/app.css`.

**Deployment**: The Node adapter (`@sveltejs/adapter-node`) builds the app for deployment as a standalone Node.js server. The build output is in the `build` directory and can be run with `node build`.

**ESLint Configuration**: Uses flat config format with TypeScript ESLint and Svelte plugin. The `no-undef` rule is disabled for TypeScript files (per typescript-eslint recommendations).

## Docker Deployment

The application includes a Dockerfile for containerized deployment:

```bash
# Build the Docker image
docker build -t lifestats-frontend .

# Run the container
docker run -p 3000:3000 lifestats-frontend
```

The Dockerfile uses a multi-stage build:
1. **Builder stage**: Installs dependencies and builds the SvelteKit app
2. **Runtime stage**: Minimal Alpine-based image with only production dependencies

The application runs on port 3000 as a non-root user for security.
