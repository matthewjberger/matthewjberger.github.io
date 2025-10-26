# Portfolio

A modern portfolio website built with Rust and WebAssembly, featuring client-side rendering with Leptos and styled with Tailwind CSS.

## Tech Stack

- **[Leptos](https://github.com/leptos-rs/leptos)** - Reactive web framework for Rust
- **[Trunk](https://github.com/trunk-rs/trunk)** - WASM web application bundler
- **[Tailwind CSS](https://tailwindcss.com/)** - Utility-first CSS framework
- **[Just](https://github.com/casey/just)** - Command runner for project tasks

## Prerequisites

- Rust nightly toolchain
- Node.js and npm
- Just command runner
- Trunk bundler

### Installation

Install Rust nightly and add the WASM target:

```sh
rustup toolchain install nightly --allow-downgrade
rustup target add wasm32-unknown-unknown
```

Install Trunk:

```sh
cargo install trunk
```

Install Just:

```sh
cargo install just
```

Install npm dependencies:

```sh
npm install
```

## Development

Start the development server:

```sh
just serve
```

This will build Tailwind CSS and start the Trunk dev server at `http://localhost:3000`.

To watch Tailwind CSS for changes in a separate terminal:

```sh
just tailwind-watch
```

## Building

Build for production:

```sh
just build
```

This builds the optimized WASM bundle and Tailwind CSS to the `dist` directory.

## Deployment

The project automatically deploys to GitHub Pages via GitHub Actions when pushing to the main branch. The workflow:

1. Runs linting (clippy and rustfmt)
2. Builds Tailwind CSS
3. Builds the WASM application with Trunk
4. Deploys to the gh-pages branch

## Project Structure

```
.
├── src/
│   ├── components/     # Reusable UI components
│   ├── pages/          # Page components (home, not_found)
│   └── lib.rs          # App entry point and router
├── public/             # Static assets
├── input.css           # Tailwind CSS input
├── index.html          # HTML template
├── Trunk.toml          # Trunk configuration
├── tailwind.config.js  # Tailwind configuration
└── justfile            # Task definitions
```

## License

MIT
