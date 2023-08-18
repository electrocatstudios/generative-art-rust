FROM rust:latest

WORKDIR /app

RUN apt update --fix-missing && \
    apt install libavutil-dev -y && \
    apt install libavcodec-dev -y && \
    apt install libavformat-dev -y && \
    apt install libswscale-dev -y && \
    apt install libavfilter-dev -y && \
    apt install libavdevice-dev -y && \  
    apt install libclang-dev -y && \ 
    apt install librust-clang-sys-dev -y && \
    apt install ffmpeg -y

COPY Cargo.toml /app/

RUN mkdir src && \
    echo 'fn main() {\nprintln!("Hello, world!");\n}' > src/main.rs && \
    cargo build && \ 
    cargo clean --package $(awk '/name/ {gsub(/"/,""); print $3}' Cargo.toml | sed ':a;N;$!ba;s/\n//g' | tr -d '\r') && \
    rm -rf src 

COPY src /app/src
COPY fonts /app/fonts

RUN cargo run

CMD [ "cp", "-r", "/app/outputs/", "/data/"]