FROM node:18-slim AS webui
WORKDIR /app/webui
COPY ./webui/package.json ./webui/package-lock.json /app/webui/
RUN npm install
COPY ./webui .
RUN npm run build

########################## SCRATCH BUILD IMAGES ##########################
## NOTE: The Alpine Base Images do not support other platforms then linux/amd64
## And for scratch we define all build images here, they will only be loaded when actually used
FROM --platform=linux/amd64 blackdex/rust-musl:x86_64-musl-stable-1.75.0 as build_amd64
FROM --platform=linux/amd64 blackdex/rust-musl:aarch64-musl-stable-1.75.0 as build_arm64
FROM --platform=linux/amd64 blackdex/rust-musl:armv7-musleabihf-stable-1.75.0 as build_armv7
FROM --platform=linux/amd64 blackdex/rust-musl:arm-musleabi-stable-1.75.0 as build_armv6

FROM --platform=linux/amd64 build_${TARGETARCH}${TARGETVARIANT} as backend
ARG TARGETARCH
ARG TARGETVARIANT
WORKDIR /app
COPY . .
ARG MK_TMDB_API
RUN cargo build --release; \
    cd target; \
    mv x86_64-unknown-linux-musl musl; \
    mv aarch64-unknown-linux-musl musl; \
    mv armv7-unknown-linux-musleabihf musl; \
    mv arm-unknown-linux-musleabi musl; \
    echo "rename build dir successfully."

FROM scratch
ENV LANG="C.UTF-8" \
    TZ=Asia/Shanghai \
    PUID=1000 \
    PGID=1000 \
    UMASK=022 \
    RUST_LOG=INFO
WORKDIR /app
COPY --chmod=755 --from=webui /app/resources ./resources
COPY --chmod=755 --from=backend /app/target/musl/release/mikanarr .
EXPOSE 7810
VOLUME [ "/app/data" ]
ENTRYPOINT [ "/app/mikanarr" ]
