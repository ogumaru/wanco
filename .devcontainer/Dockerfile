FROM busybox:1.34.1 as nodejs
ENV NODEJS_TARBALL_URL="https://nodejs.org/dist/v18.12.1/node-v18.12.1-linux-x64.tar.xz"
USER root
WORKDIR /root/.local
RUN mkdir -p bin/ node/ \
    && wget -O node.tar.xz "${NODEJS_TARBALL_URL}" --no-check-certificate \
    && xz -d -c node.tar.xz | tar xvf - -C node --strip-components 1 \
    && ln -s "/root/.local/node/bin/node" bin/ \
    && ln -s "/root/.local/node/bin/npm" bin/ \
    && ln -s "/root/.local/node/bin/npx" bin/ \
    && rm node.tar.xz


FROM busybox:1.34.1 as firefox
ENV FIREFOX_TARBALL_URL="https://download-installer.cdn.mozilla.net/pub/firefox/releases/106.0.5/linux-x86_64/en-US/firefox-106.0.5.tar.bz2"
USER root
WORKDIR /root/.local
RUN mkdir -p bin \
    && wget -O firefox.tar.bz2 "${FIREFOX_TARBALL_URL}" \
    && bzcat firefox.tar.bz2 | tar xvf - \
    && ln -s "$(readlink -f firefox/firefox)" bin/ \
    && rm firefox.tar.bz2

# -----------------------------------------------------------------------------

FROM mcr.microsoft.com/vscode/devcontainers/rust:0-bullseye as rust
ENV PATH="${PATH}:/root/.local/" \
    DEBIAN_FRONTEND=noninteractive
COPY --from=firefox /root/.local /root/.local
COPY --from=nodejs /root/.local /root/.local
RUN cargo install wasm-pack \
    && apt update \
    && apt install -y --no-install-recommends \
        libnss3 \
        libxcb1 \
        libgtk-3-0 \
        libasound2 \
        libdbus-glib-1-2 \
        libx11-xcb1 \
        libxtst6 \
    && apt clean \
    && rm -rf /var/lib/apt/lists/*
