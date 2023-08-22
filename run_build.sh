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
  WIDTH=$(awk '/const WIDTH: u32 = / {gsub(/;/,""); print $5}' src/main.rs  | tr -d '\r')
  HEIGHT=$(awk '/const HEIGHT: u32 = / {gsub(/;/,""); print $5}' src/main.rs  | tr -d '\r')
  
  # | sed ':a;N;$!ba;s/\n//g' 
  cp templates/index.html data/out/index.html

  sed -i '' "s/<<WIDTH>>/$WIDTH/" "data/out/index.html"
  sed -i '' "s/<<HEIGHT>>/$HEIGHT/" data/out/index.html
fi

# docker rm $(docker container ls -a -q --filter ancestor=gen_rust --filter status=exited)