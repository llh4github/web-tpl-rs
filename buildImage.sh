APP_NAME=my-rust-api
GIT_COMMIT=$(git describe --tags --always)
docker build -t linhong4dockerhub/${APP_NAME}:latest .
#docker tag linhong4dockerhub/${APP_NAME}:latest linhong4dockerhub/${APP_NAME}:git-${GIT_COMMIT}

docker rmi $(docker images -q -f "dangling=true") || true
