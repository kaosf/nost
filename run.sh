set -ex

docker container run -d \
-v $PWD/config:/workspace/config:ro \
-v $PWD/data:/workspace/data \
-u $UID \
-e RUST_LOG=debug \
--name nost \
kaosf/nost:3.3.2

# # restart=always
# docker container run -d --restart=always \
# -v $PWD/config:/workspace/config:ro \
# -v $PWD/data:/workspace/data \
# -u $UID \
# -e RUST_LOG=debug \
# --name nost \
# kaosf/nost:3.3.2

# Log viewer
docker container logs -f nost

# Kill and remove
docker container kill nost
docker container rm -f nost
