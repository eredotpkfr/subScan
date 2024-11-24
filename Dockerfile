FROM rust:1.82-slim-bookworm AS builder

WORKDIR /app

ENV DEBIAN_FRONTEND=noninteractive

# Install chrome and project deps
RUN apt-get update -y && \
    apt-get install -y --no-install-recommends \
    libasound2 \
    libatk-bridge2.0-0 \
    libatk1.0-0 \
    libcups2 \
    libdrm2 \
    libgbm1 \
    libglib2.0-0 \
    libnss3 \
    libpango-1.0-0 \
    libpangocairo-1.0-0 \
    libssl-dev \
    libxcomposite1 \
    libxdamage1 \
    libxfixes3 \
    libxkbcommon0 \
    libxrandr2 \
    pkg-config \
    && rm -rf /var/lib/apt/lists/*

COPY Cargo.toml .
COPY src src

RUN cargo build --release

FROM gcr.io/distroless/cc-debian12:nonroot

# Copy libs from builder
COPY --from=builder /lib /lib
COPY --from=builder /usr/share /usr/share
# Copy subscan binary
COPY --from=builder --chown=nonroot /app/target/release/subscan /app/subscan

ENTRYPOINT [ "/app/subscan" ]
