FROM rust:1.83.0-alpine3.21 AS builder

RUN apk add --no-cache \
    musl-dev \
    openssl-dev

# Update package repositories
RUN apk update

WORKDIR /usr/src/dyncloud
COPY . .

RUN cargo build --release --features "mimalloc"

# Final stage: create a minimal runtime image
FROM alpine:3.21.2

# Update package repositories
RUN apk add --no-cache curl
RUN apk update

# Create a non-root user
RUN addgroup -g 1000 -S dyncloud && \
    adduser -u 1000 -S -D -G dyncloud -H -h /home/dyncloud -s /bin/sh dyncloud

# Set the working directory
WORKDIR /home/dyncloud

# Copy the binaries from the builder stage
COPY --from=builder --chown=1000:1000 /usr/src/dyncloud/target/release/dyncloud /home/dyncloud/dyncloud

# Set permissions for the binary
RUN chmod +x /home/dyncloud/dyncloud

# Switch to non-root user
USER dyncloud

# Entrypoint command
ENTRYPOINT ["/home/dyncloud/dyncloud"]
