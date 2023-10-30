# FROM istio/distroless
# FROM alpine:3 # I want to use them for lightweight image.
FROM ubuntu:latest
WORKDIR /workspace/
COPY ["target/release/nost", "/bin/"]
ENTRYPOINT ["nost"]
