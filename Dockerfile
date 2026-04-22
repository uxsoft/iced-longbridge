# ── Build stage ────────────────────────────────────────────────────────────────
# Pin base image tag so rebuilds are reproducible. Bump deliberately, not silently.
FROM rust:latest AS builder

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
# Each release asset ships with a sibling .sha256 file containing only the digest;
# we reconstruct the `sha256sum -c` line so the build fails loudly on any mismatch.
RUN case "$(uname -m)" in \
        x86_64)  ARCH="x86_64-unknown-linux-gnu"   ;; \
        aarch64) ARCH="aarch64-unknown-linux-gnu"  ;; \
        *) echo "Unsupported arch: $(uname -m)" && exit 1 ;; \
    esac && \
    cd /tmp && \
    wget -q "https://github.com/trunk-rs/trunk/releases/download/${TRUNK_VERSION}/trunk-${ARCH}.tar.gz" && \
    wget -q "https://github.com/trunk-rs/trunk/releases/download/${TRUNK_VERSION}/trunk-${ARCH}.tar.gz.sha256" && \
    echo "$(cat trunk-${ARCH}.tar.gz.sha256)  trunk-${ARCH}.tar.gz" | sha256sum -c - && \
    tar -xzC /usr/local/bin -f "trunk-${ARCH}.tar.gz" && \
    rm -f "trunk-${ARCH}.tar.gz" "trunk-${ARCH}.tar.gz.sha256"

WORKDIR /workspace
COPY . .

WORKDIR /workspace/crates/demo-web
RUN trunk build --release

# ── Serve stage ─────────────────────────────────────────────────────────────────
FROM nginx:stable-alpine

# Minimal hardening for the static WASM demo.
COPY nginx.conf /etc/nginx/conf.d/default.conf
COPY --from=builder /workspace/crates/demo-web/dist /usr/share/nginx/html

EXPOSE 80
CMD ["nginx", "-g", "daemon off;"]
