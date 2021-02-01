.PHONY: build-web-frontend build-webplay-frontend serve-webplay format

build-webplay-backend:
	cd webplay/backend; cargo build

build-webplay-frontend:
	cd webplay/frontend; wasm-pack build --target web

serve-webplay:
	cargo run -p webplay-backend -- webplay/frontend

format:
	cargo fmt --all

watch-webplay-frontend:
	fswatch -0 webplay/frontend/src | xargs -0 -n 1 -I {} make build-webplay-frontend

test:
	cargo test
