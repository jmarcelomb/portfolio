FROM rust:1-bookworm AS builder

WORKDIR /workspace

RUN apt update && \
	apt install -y \
	npm

RUN rustup default nightly 
RUN rustup target add wasm32-unknown-unknown

RUN cargo install --locked cargo-leptos
RUN npm install tailwindcss -g

COPY . .

FROM builder AS development
RUN cargo install --locked trunk
USER vscode
RUN export CARGO_HOME=$HOME/.cargo
EXPOSE 8080

FROM builder AS production
RUN npx tailwindcss -i style/tailwind.css -o style/output.css --minify
RUN cargo leptos build --release
