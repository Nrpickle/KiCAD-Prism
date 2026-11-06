# Build the native fix over the exact backend release already deployed.
FROM rust:1.98.0-bookworm@sha256:82150a52ec202c1b14d7817e14516c392bb7f5cfebd88f1ed531cb37ebd39922 AS native
WORKDIR /build/native
ENV RUSTUP_TOOLCHAIN=1.98.0
COPY native/Cargo.toml native/Cargo.lock native/rust-toolchain.toml native/clippy.toml ./
COPY native/.cargo ./.cargo
COPY native/src/rs ./src/rs
COPY native/assets ./assets
COPY native/third_party ./third_party
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/build/native/target \
    cargo build --locked --release -j 2 -p prism-kicad-cli \
    && cp target/release/prism-kicad /tmp/prism-kicad

FROM ghcr.io/krishna-swaroop/kicad-prism-backend@sha256:c6a84098f49d269449080ae257145db1445c3e9c156120b5e11bdb56b3d42256
ARG SOURCE_REVISION
LABEL org.opencontainers.image.source="https://github.com/Nrpickle/KiCAD-Prism" \
      org.opencontainers.image.revision="${SOURCE_REVISION}" \
      io.kicad-prism.local-patch="missing-pin-uuid-validation"
COPY --from=native /tmp/prism-kicad /usr/local/bin/prism-kicad
COPY native/contracts/generated/schema/SemanticIndexA3.json /app/native/contracts/generated/schema/SemanticIndexA3.json
