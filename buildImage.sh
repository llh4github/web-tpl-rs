docker build -t my-rust-api .

docker rmi $(docker images -q -f "dangling=true") || true
