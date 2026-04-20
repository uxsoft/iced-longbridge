# ── Build stage ────────────────────────────────────────────────────────────────
FROM rust:slim-bookworm AS builder

RUN apt-get update && apt-get install -y wget && rm -rf /var/lib/apt/lists/*

# WASM compilation target
RUN rustup target add wasm32-unknown-unknown

# Install Trunk from the pre-built binary (avoids a slow `cargo install` compile).
# Detect the host architecture at build time so the image builds on amd64 and arm64.
RUN case "$(uname -m)" in \
        x86_64)  ARCH="x86_64-unknown-linux-gnu"   ;; \
        aarch64) ARCH="aarch64-unknown-linux-gnu"  ;; \
        *) echo "Unsupported arch: $(uname -m)" && exit 1 ;; \
    esac && \
    wget -qO- \
        "https://github.com/trunk-rs/trunk/releases/latest/download/trunk-${ARCH}.tar.gz" \
    | tar -xzC /usr/local/bin

WORKDIR /workspace
COPY . .

WORKDIR /workspace/crates/demo-web
RUN trunk build --release

# ── Serve stage ─────────────────────────────────────────────────────────────────
FROM nginx:alpine
COPY --from=builder /workspace/crates/demo-web/dist /usr/share/nginx/html
EXPOSE 80
CMD ["nginx", "-g", "daemon off;"]
