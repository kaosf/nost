FROM rust:1-bookworm AS builder
WORKDIR /workspace/
COPY ["Cargo.toml", "Cargo.lock", "/workspace/"]
COPY ["src/", "/workspace/src/"]
RUN cargo build --release

FROM gcr.io/distroless/base
COPY --from=builder ["/lib/x86_64-linux-gnu/libgcc_s.so.1", "/lib/x86_64-linux-gnu/libgcc_s.so.1"]
COPY --from=builder ["/lib/x86_64-linux-gnu/libm.so.6", "/lib/x86_64-linux-gnu/libm.so.6"]
COPY --from=builder ["/workspace/target/release/nost", "/bin/nost"]
WORKDIR /workspace/
ENTRYPOINT ["nost"]
