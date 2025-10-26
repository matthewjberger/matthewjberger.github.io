tailwind:
    npx tailwindcss -i ./input.css -o ./public/styles.css --minify

tailwind-watch:
    npx tailwindcss -i ./input.css -o ./public/styles.css --watch

serve: tailwind
    trunk serve --port 3000 --open

build: tailwind
    trunk build --release
