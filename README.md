# Portfolio

Working with Rust since 2016, with 5 years in production robotics at Hyphen Robotics and earlier production Rust at Sierra Nevada Corporation (aerospace imaging). Background also includes safety-critical medical robotics at Hamilton Company.

## Links

- [matthewberger.dev](https://matthewberger.dev) portfolio site
- [LinkedIn](https://www.linkedin.com/in/matthewjberger/)
- [Resume (PDF)](static/Berger_Matthew_Resume.pdf)

## About

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
└── static/                 # Copied verbatim to the output root (Berger_Matthew_Resume.pdf, images, CNAME, favicon)
```

All content lives in `content/` and `data/`. The layout, interactivity, and styling come from bamboo's built-in `portfolio.html` template.

## Resume

`resume.tex` is the source for `static/Berger_Matthew_Resume.pdf`. Rebuild it with `pdflatex`:

```sh
pdflatex resume.tex
mv resume.pdf static/Berger_Matthew_Resume.pdf
```

Install `pdflatex` via:

- **Windows**: `scoop install miktex` (from [scoop.sh](https://scoop.sh))
- **macOS**: `brew install --cask mactex-no-gui`
- **Linux**: `sudo apt install texlive-latex-base` (Debian/Ubuntu) or equivalent

## Deployment

GitHub Actions (`.github/workflows/gh-pages.yml`) installs `bamboo-cli`, builds the site with `--base-url "https://matthewberger.dev"`, and publishes to GitHub Pages on every push to `main`.

## License

MIT. See [LICENSE.md](LICENSE.md).
