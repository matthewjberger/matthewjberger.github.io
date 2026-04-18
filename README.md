# Portfolio

Personal portfolio site at [matthewberger.dev](https://matthewberger.dev), built with [bamboo](https://github.com/matthewjberger/bamboo), a static site generator written in Rust.

## Prerequisites

```sh
cargo install bamboo-cli
cargo install just
```

## Development

```sh
just serve   # live-reload server at http://localhost:3000
just build   # one-shot build to dist/
```

## Structure

```
.
├── bamboo.toml             # Site config + [extra] fields consumed by portfolio template
├── content/
│   ├── _index.md           # Home (About) copy + template selection
│   └── projects/           # One markdown file per project
├── data/                   # TOML data files consumed by the portfolio template
│   ├── experience.toml
│   ├── education.toml
│   ├── highlights.toml
│   └── crates.toml
└── static/                 # Copied verbatim to the output root (Resume.pdf, images, CNAME, favicon)
```

All content lives in `content/` and `data/`. The layout, interactivity, and styling come from bamboo's built-in `portfolio.html` template.

## Deployment

GitHub Actions (`.github/workflows/gh-pages.yml`) installs `bamboo-cli`, builds the site with `--base-url "https://matthewberger.dev"`, and publishes to GitHub Pages on every push to `main`.

## License

MIT. See [LICENSE.md](LICENSE.md).
