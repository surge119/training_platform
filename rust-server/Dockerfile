FROM ubuntu:22.04

WORKDIR /opt/rust-server

RUN apt-get update \
	&& apt-get -y install \
	build-essential \
	curl \
	git \
	&& apt-get clean \
	&& rm -rf /var/lib/apt/lists/*

COPY . /opt/rust-server

RUN adduser \
	--disabled-login \
	-u 1001 \
	--gecos "" \
	--shell /bin/bash \
	rust-server \
	&& chmod +x /opt/rust-server/docker-entrypoint.sh \
	&& chown -R 1001:1001 /opt/rust-server

RUN runuser -l  rust-server -c 'curl https://sh.rustup.rs -sSf | bash -s -- -y'

RUN runuser -l rust-server -c 'cargo build --release --manifest-path /opt/rust-server/Cargo.toml'

USER 1001
EXPOSE 8000
ENTRYPOINT ["/opt/rust-server/docker-entrypoint.sh"]
