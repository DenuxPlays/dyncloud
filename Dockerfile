FROM rust:1.90.0-alpine3.22 AS builder

RUN apk add --no-cache \
    musl-dev \
    openssl-dev && \
    apk update

WORKDIR /usr/src/dyncloud
COPY . .

RUN cargo build --release --features "mimalloc"

# Final stage: create a minimal runtime image
FROM alpine:3.22.2

# Update package repositories
RUN apk add --no-cache curl tzdata && \
    apk update

# User creation
ARG UID=1000
ARG GUID=1000
ARG USER_NAME=dyncloud
ENV GROUP_NAME=${USER_NAME}
ENV HOME_DIR=/home/${USER_NAME}

# Create a non-root user
RUN addgroup -g ${UID} -S ${GROUP_NAME} && \
    adduser -u ${GUID} -S -D -G ${GROUP_NAME} -h ${HOME_DIR} -s /bin/sh ${USER_NAME}

# Set the working directory
WORKDIR ${HOME_DIR}

# Copy the binaries from the builder stage
COPY --from=builder --chown=${UID}:${GUID} --chmod=+x /usr/src/dyncloud/target/release/dyncloud /home/dyncloud/dyncloud

# Switch to non-root user
USER ${USER_NAME}

ENTRYPOINT ["/home/dyncloud/dyncloud"]

CMD ["run"]
