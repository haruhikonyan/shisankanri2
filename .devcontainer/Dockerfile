FROM mcr.microsoft.com/devcontainers/rust:0-1-bullseye
RUN apt update && apt install -y chromium
USER vscode
RUN curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
RUN cargo install cargo-watch
