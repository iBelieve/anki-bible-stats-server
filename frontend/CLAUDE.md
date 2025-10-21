# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is a SvelteKit application configured to deploy to Cloudflare using the Cloudflare adapter. The project uses:
- **SvelteKit** with Svelte 5 (using new runes syntax like `$props()`)
- **TypeScript** for type safety
- **Tailwind CSS v4** (via Vite plugin) for styling
- **Cloudflare Pages** as the deployment target

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
npm run build            # Production build for Cloudflare
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

**Deployment**: The Cloudflare adapter (`@sveltejs/adapter-cloudflare`) builds the app for Cloudflare Pages/Workers. Build output is optimized for edge deployment.

**ESLint Configuration**: Uses flat config format with TypeScript ESLint and Svelte plugin. The `no-undef` rule is disabled for TypeScript files (per typescript-eslint recommendations).
