# ── Build stage ────────────────────────────────────────────────────────────────
# Pin base image tag so rebuilds are reproducible. Bump deliberately, not silently.
FROM rust:1.84-slim-bookworm AS builder

# Trunk release pinned to a specific version. The checksum manifest published
# alongside the release is used to verify the tarball before extracting it.
ARG TRUNK_VERSION=v0.21.14

RUN apt-get update && apt-get install -y --no-install-recommends \
        wget ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# WASM compilation target
RUN rustup target add wasm32-unknown-unknown

# Install Trunk from the pre-built binary (avoids a slow `cargo install` compile).
# Detect the host architecture at build time so the image builds on amd64 and arm64.
# The sha256-manifest.txt file is an artifact of the Trunk release; if it's missing
# or doesn't match, the build fails loudly rather than silently accepting a swap.
RUN case "$(uname -m)" in \
        x86_64)  ARCH="x86_64-unknown-linux-gnu"   ;; \
        aarch64) ARCH="aarch64-unknown-linux-gnu"  ;; \
        *) echo "Unsupported arch: $(uname -m)" && exit 1 ;; \
    esac && \
    cd /tmp && \
    wget -q "https://github.com/trunk-rs/trunk/releases/download/${TRUNK_VERSION}/trunk-${ARCH}.tar.gz" && \
    wget -q "https://github.com/trunk-rs/trunk/releases/download/${TRUNK_VERSION}/sha256-manifest.txt" && \
    grep " trunk-${ARCH}.tar.gz$" sha256-manifest.txt | sha256sum -c - && \
    tar -xzC /usr/local/bin -f "trunk-${ARCH}.tar.gz" && \
    rm -f "trunk-${ARCH}.tar.gz" sha256-manifest.txt

WORKDIR /workspace
COPY . .

WORKDIR /workspace/crates/demo-web
RUN trunk build --release

# ── Serve stage ─────────────────────────────────────────────────────────────────
FROM nginx:1.27-alpine

# Minimal hardening for the static WASM demo.
COPY nginx.conf /etc/nginx/conf.d/default.conf
COPY --from=builder /workspace/crates/demo-web/dist /usr/share/nginx/html

EXPOSE 80
CMD ["nginx", "-g", "daemon off;"]
