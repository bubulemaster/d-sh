FROM ubuntu:18.04

ENV RUSTUP_HOME /opt/rustup
ENV CARGO_HOME /opt/cargo
ENV CARGO_BIN /opt/cargo/bin

RUN apt-get update && \
    apt-get install -y \
    curl bash build-essential sudo

RUN curl https://sh.rustup.rs -sSf -o /tmp/install-rust.sh && \
    chmod u+x /tmp/install-rust.sh && \
    /tmp/install-rust.sh --no-modify-path -v -y && \
    ${CARGO_BIN}/rustup install 1.32.0 && \
    ${CARGO_BIN}/rustup component add rustfmt

RUN mkdir ${CARGO_HOME}/registry && \
    chmod a+w -R ${CARGO_HOME}/registry

ENV PATH "$PATH:${CARGO_BIN}"

COPY entrypoint.sh /entrypoint.sh

ENTRYPOINT ["/bin/sh", "/entrypoint.sh"]
