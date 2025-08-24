# Bevy Canvas with Leptos Lazy Routes Issue Reproduction

This project demonstrates an issue with Bevy canvas components when using lazy routes in Leptos 0.8.6.

## Architecture

- **Leptos 0.8.6** with SSR enabled
- **Bevy 0.16** canvas running only on client-side (hydrate feature)
- **leptos-bevy-canvas 0.3.0** for Bevy/Leptos integration
- All routes configured for lazy loading with code splitting

## Routes

1. **/** - Home page with Bevy canvas
2. **/about** - Simple about page
3. **/contact** - Contact form page
4. **/dashboard** - Dashboard with toggleable Bevy canvas

## Building and Running

### Prerequisites

```bash
# Install cargo-leptos
cargo install cargo-leptos

# Install just (optional, for using justfile commands)
cargo install just
```

### Using justfile

```bash
# Build with code splitting
just build-split

# Run development server with code splitting
just dev-split

# Build for production with code splitting
just build-release

# Diagnose build issues with verbose output
just diagnose
```

### Manual commands

```bash
# Development mode
cargo leptos watch

# Build with code splitting
cargo leptos build --split

# Production build with code splitting
cargo leptos build --release --split
```

## Issue Description

When using lazy routes with code splitting (`--split` flag), the Bevy canvas component may not properly initialize or render on the client side after hydration. This reproduction helps identify and debug the issue.

## Key Files

- `src/components/bevy_canvas.rs` - Bevy canvas component with hydrate-only rendering
- `src/pages/home.rs` - Home page using the Bevy canvas
- `src/pages/dashboard.rs` - Dashboard with toggleable canvas
- `src/app.rs` - Route configuration
- `Cargo.toml` - Dependencies and feature configuration