#!/bin/bash

# Install python
sudo apt install -y python3

# Install docker
curl -fsSL https://get.docker.com -o get-docker.sh && sudo sh get-docker.sh

# Stop docker daemon
sudo systemctl stop docker
sudo systemctl stop docker.socket

# Start docker daemon and listen on 10.10.10.1
sudo dockerd -H unix:///var/run/docker.sock -H tcp://10.10.10.1 > dockerd-logs &

# Add current user to docker group
sudo usermod -aG docker $(whomai)

