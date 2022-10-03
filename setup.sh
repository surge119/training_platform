#!/bin/bash

# Install python
sudo apt install -y python3 pip

pip install -r requirements.txt

# Install docker
curl -fsSL https://get.docker.com -o get-docker.sh && sudo sh get-docker.sh

# Stop docker daemon
sudo systemctl stop docker
sudo systemctl stop docker.socket

# Add current user to docker group
sudo usermod -aG docker $(whoami)

# Start docker daemon and listen on 172.17.0.1 (docker ip)
sudo dockerd -H unix:///var/run/docker.sock -H tcp://172.17.0.1 > dockerd-logs &

