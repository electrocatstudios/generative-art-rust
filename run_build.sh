# Add --progress=plain to output build log
docker build -t gen_rust .

docker run \
  -it \
  --mount type=bind,source="$(pwd)"/data,target=/data \
  gen_rust

docker rm $(docker container ls -a -q --filter ancestor=gen_rust --filter status=exited)