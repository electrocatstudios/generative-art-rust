docker build -t gen_rust .

docker run \
  -it \
  --mount type=bind,source="$(pwd)"/data,target=/data \
  gen_rust
