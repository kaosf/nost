FROM gcr.io/distroless/base
WORKDIR /workspace/
COPY ["target/release/nost", "/bin/"]
ENTRYPOINT ["nost"]
