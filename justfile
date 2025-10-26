# Displays the list of available commands
@just:
    just --list

# Installs npm dependencies
install:
    npm install

# Builds Tailwind CSS styles
tailwind: install
    npx tailwindcss -i ./input.css -o ./public/styles.css --minify

# Watches for changes and rebuilds Tailwind CSS styles
tailwind-watch:
    npx tailwindcss -i ./input.css -o ./public/styles.css --watch

# Serves the application locally on port 3000
serve: tailwind
    trunk serve --port 3000 --open

# Builds the project for release
build: tailwind
    trunk build --release

# Runs cargo check and format check
check:
    cargo check --all --tests
    cargo fmt --all -- --check

# Formats the code using cargo fmt
format:
    cargo fmt
