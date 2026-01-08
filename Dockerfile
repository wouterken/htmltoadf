FROM debian:bookworm-slim AS bin
ARG VERSION
ARG TARGETARCH

COPY releases/${VERSION}/ /tmp/releases/
RUN case "${TARGETARCH}" in \
        amd64) target_dir="x86_64-unknown-linux-gnu" ;; \
        arm64) target_dir="aarch64-unknown-linux-gnu" ;; \
        *) echo "Unsupported TARGETARCH=${TARGETARCH}" && exit 1 ;; \
    esac && \
    install -m 0755 "/tmp/releases/${target_dir}/html2adf" /html2adf

FROM debian:bookworm-slim
COPY --from=bin /html2adf /usr/local/bin/html2adf

ENTRYPOINT ["html2adf"]
CMD []
