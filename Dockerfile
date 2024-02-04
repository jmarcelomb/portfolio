FROM rust:1-bookworm AS builder

WORKDIR /workspace

RUN apt update && \
	apt install -y \
	npm

RUN rustup default nightly 
RUN rustup target add wasm32-unknown-unknown

RUN cargo install --locked cargo-leptos
RUN npm install tailwindcss -g

FROM builder AS development
RUN cargo install --locked trunk 
RUN cargo install leptosfmt
# Create the user
ARG USERNAME=vscode
ARG USER_UID=1000
ARG USER_GID=$USER_UID
RUN groupadd --gid $USER_GID $USERNAME \
    && useradd --uid $USER_UID --gid $USER_GID -m $USERNAME \
    # [Optional] Add sudo support. Omit if you don't need to install software after connecting.
    && apt-get update \
    && apt-get install -y sudo \
    && echo $USERNAME ALL=\(root\) NOPASSWD:ALL > /etc/sudoers.d/$USERNAME \
    && chmod 0440 /etc/sudoers.d/$USERNAME

# [Optional] Set the default user. Omit if you want to keep the default as root.
USER $USERNAME
RUN sh -c "$(wget -O- https://github.com/deluan/zsh-in-docker/releases/download/v1.1.5/zsh-in-docker.sh)" -- \
    -t robbyrussell \
    -p git \
    -p https://github.com/zsh-users/zsh-autosuggestions \
    -p https://github.com/zsh-users/zsh-completions
RUN echo "export CARGO_HOME=$HOME/.cargo" >> ~/.bashrc
RUN echo "export CARGO_HOME=$HOME/.cargo" >> ~/.zshrc || true

EXPOSE 8080

FROM builder AS production
RUN npx tailwindcss -i input.css -o style/output.css --minify
COPY . .
RUN cargo leptos build --release

CMD ["sleep", "infinity"]
