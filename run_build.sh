# Add --progress=plain to output build log
DOCKERFILE="-f Dockerfile"
IS_WEB=false

for arg in "$@"
do 
  case $arg in
    "--web" )
      DOCKERFILE="-f Dockerfile.web"
      IS_WEB=true
  esac
done

docker build -t gen_rust $DOCKERFILE .

docker run \
  -it \
  --mount type=bind,source="$(pwd)"/data,target=/data \
  gen_rust

if [ $IS_WEB = true ]; then
  cp templates/index.html data/out/index.html
fi

# docker rm $(docker container ls -a -q --filter ancestor=gen_rust --filter status=exited)