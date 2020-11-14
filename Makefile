.PHONY: build-web-frontend

build-webplay-backend:
	cd webplay/backend; cargo build

build-webplay-frontend:
	cd webplay/frontend; wasm-pack build --target web

serve-webplay:
	cargo run -p webplay-backend -- webplay/frontend
