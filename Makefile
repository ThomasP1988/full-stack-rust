run-api:
	cargo run -p api
run-web:
	cd web && trunk serve
watch-style:
	cd web && npx tailwindcss -i ./styles/input.css -o ./styles/output.css --watch
build-web:
	cargo build -p web --target wasm32-unknown-unknown