FROM python:3.9-slim-buster
WORKDIR /opt/CTFd
RUN mkdir -p /opt/CTFd /var/log/CTFd /var/uploads /opt/training_plat

# hadolint ignore=DL3008
RUN apt-get update \
    && apt-get install -y --no-install-recommends \
        build-essential \
        libffi-dev \
        libssl-dev \
        git \
        curl \
    && apt-get clean \
    && rm -rf /var/lib/apt/lists/*

COPY ./CTFd-3.5.0/requirements.txt /opt/CTFd/

RUN pip install -r requirements.txt --no-cache-dir

COPY /CTFd-3.5.0 /opt/CTFd
COPY . /opt/training_plat
RUN rm -r /opt/training_plat/CTFd-3.5.0



# hadolint ignore=SC2086
RUN for d in CTFd/plugins/*; do \
        if [ -f "$d/requirements.txt" ]; then \
            pip install -r $d/requirements.txt --no-cache-dir; \
        fi; \
    done;

# hadolint ignore=DL3059
RUN adduser \
    --disabled-login \
    -u 1001 \
    --gecos "" \
    --shell /bin/bash \
    ctfd \
    && chmod +x /opt/CTFd/docker-entrypoint.sh \
    && chown -R 1001:1001 /opt/CTFd /var/log/CTFd /var/uploads /opt/training_plat
# Get Rust
RUN runuser -l  ctfd -c 'curl https://sh.rustup.rs -sSf | bash -s -- -y'

RUN runuser -l ctfd -c 'cargo build --manifest-path /opt/training_plat/Cargo.toml'
RUN chmod +x /opt/training_plat/target/debug/training_platform
USER 1001
EXPOSE 8000
ENTRYPOINT ["/opt/CTFd/docker-entrypoint.sh"]
