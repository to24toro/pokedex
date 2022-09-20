ARG VARIANT="bullseye"
FROM mcr.microsoft.com/devcontainers/rust:1-${VARIANT}
ENV CARGO_BUILD_TARGET_DIR=/tmp/target
WORKDIR /home/rust
COPY . .

# sqlite3のインストール
RUN apt-get install sqlite3 libsqlite3-dev -y

