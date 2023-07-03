FROM rust:latest

WORKDIR /app
COPY Cargo.toml /app/
COPY src /app/src

RUN apt update && \
    apt install libavutil-dev -y && \
    apt install libavcodec-dev -y && \
    apt install libavformat-dev -y && \
    apt install libswscale-dev -y && \
    apt install ffmpeg -y

RUN cargo run
ENTRYPOINT [ "cp", "/app/gifs/output.gif", "/data/" ]