# Bevy Canvas with Leptos Lazy Routes Issue Reproduction

This project demonstrates an issue with Bevy canvas components when using lazy routes in Leptos 0.8.6.

## Architecture

- **Leptos 0.8.6** with SSR enabled
- **Bevy 0.16** canvas running only on client-side (hydrate feature)
- **leptos-bevy-canvas 0.3.0** for Bevy/Leptos integration
- LazyRoute trait implementation for code splitting

## Current Implementation

### Routes

1. **/** - Home page with Bevy canvas (lazy loaded)
2. **/about** - Simple about page

### LazyRoute Implementation

The home page is implemented as a lazy route using the LazyRoute trait:

```rust
pub struct HomePage;

#[lazy_route]
impl LazyRoute for HomePage {
    fn data() -> Self {
        HomePage
    }
    
    fn view(this: Self) -> AnyView {
        // Component view implementation
    }
}
```

Routes are registered with the `Lazy` wrapper:
```rust
<Route path=path!("/") view={Lazy::<pages::home::HomePage>::new()}/>
```

Hydration uses `hydrate_lazy` instead of `hydrate_body`:
```rust
leptos::mount::hydrate_lazy(App);
```

## Known Issue

When using Bevy with Leptos code splitting (`--split` flag), you get this error in the browser console:

```
Failed to resolve module specifier "__wbindgen_placeholder__". 
Relative references must start with either "/", "./", or "../".
```

## Building and Running

### Prerequisites

```bash
# Install cargo-leptos (use the version with max-filename-length fix if needed)
cargo binstall --git https://github.com/leptos-rs/cargo-leptos --branch max-filename-length cargo-leptos

# Install just (optional, for using justfile commands)
cargo install just
```

### Using justfile

```bash
# Run debug build with code splitting
just run debug

# Build with code splitting
just compile debug

# Clean build artifacts
just clean
```

### Manual commands

```bash
# Development mode
cargo leptos serve

# Build with code splitting
cargo leptos build --split

# Production build with code splitting
cargo leptos build --release --split
```


## Key Files

- `src/components/bevy_canvas.rs` - Bevy canvas component with hydrate-only rendering
- `src/pages/home.rs` - Home page using the Bevy canvas
- `src/pages/dashboard.rs` - Dashboard with toggleable canvas
- `src/app.rs` - Route configuration
- `Cargo.toml` - Dependencies and feature configuration