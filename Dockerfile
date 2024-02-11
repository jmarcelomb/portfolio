FROM rust:1-bookworm AS builder

WORKDIR /workspace

RUN apt update && \
	apt install -y \
	npm

RUN rustup default nightly 
RUN rustup target add wasm32-unknown-unknown wasm32-wasi

RUN npm install tailwindcss -g

FROM builder AS development
RUN cargo install --locked trunk 
RUN cargo install leptosfmt
RUN sh -c "$(wget -O- https://github.com/deluan/zsh-in-docker/releases/download/v1.1.5/zsh-in-docker.sh)" -- \
    -t robbyrussell \
    -p git \
    -p https://github.com/zsh-users/zsh-autosuggestions \
    -p https://github.com/zsh-users/zsh-completions

EXPOSE 8080

FROM builder AS production
RUN cargo install --locked cargo-leptos
RUN npx tailwindcss -i input.css -o style/output.css --minify
COPY . .
RUN cargo leptos build --release

CMD ["sleep", "infinity"]
