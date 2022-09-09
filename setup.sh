#!/bin/bash

# Install python
sudo apt install -y python3

# Install docker
curl -fsSL https://get.docker.com -o get-docker.sh && sudo sh get-docker.sh

# Start docker daemon and listen on 10.10.10.1
sudo dockerd -H unix:///var/run/docker.sock -H tcp://10.10.10.1 > dockerd-logs &
