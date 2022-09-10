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

COPY ./requirements.txt /opt/CTFd/

RUN pip install -r requirements.txt --no-cache-dir

COPY . /opt/CTFd

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

#Setup Docker Network

USER 1001
EXPOSE 4000
ENTRYPOINT ["/opt/CTFd/docker-entrypoint.sh"]
