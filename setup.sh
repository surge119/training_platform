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
sudo usermod -aG docker $(whomai)

# Start docker daemon and listen on 172.17.0.1 (docker ip)
sudo dockerd -H unix:///var/run/docker.sock -H tcp://172.17.0.1 > dockerd-logs &

# HTTPS
email="umasscybersec@gmail.com"
data_path="./data/certbot"
domains=(test.training.umasscybersec.org)
rsa_key_size=4096
staging=1

curl -o https://raw.githubusercontent.com/certbot/certbot/master/certbot-nginx/certbot_nginx/_internal/tls_configs/options-ssl-nginx.conf "$data_path/conf/options-ssl-nginx.conf"
curl -o https://raw.githubusercontent.com/certbot/certbot/master/certbot/certbot/ssl-dhparams.pem "$data_path/conf/ssl-dhparams.pem"

# Create dummy certificate
echo "### Creating dummy certificate for $domains ..."
path="/etc/letsencrypt/live/$domains"
mkdir -p "$data_path/conf/live/$domains"
docker-compose run --rm --entrypoint "\
  openssl req -x509 -nodes -newkey rsa:$rsa_key_size -days 1\
    -keyout '$path/privkey.pem' \
    -out '$path/fullchain.pem' \
    -subj '/CN=localhost'" certbot
echo

# Start CTFd
echo "[+] Starting CTFd"
docker compose up --force-recreate -d ctfd

# Start nginx
echo "[+] Starting nginx"
docker compose up --force-recreate -d nginx

# Delete dummy certs
echo "### Deleting dummy certificate for $domains ..."
docker-compose run --rm --entrypoint "\
  rm -Rf /etc/letsencrypt/live/$domains && \
  rm -Rf /etc/letsencrypt/archive/$domains && \
  rm -Rf /etc/letsencrypt/renewal/$domains.conf" certbot

# Request new certificates
echo "### Requesting Let's Encrypt certificate for $domains ..."
domain_args=""
for domain in "${domains[@]}"; do
  domain_args="$domain_args -d $domain"
done

# Select appropriate email arg
case "$email" in
  "") email_arg="--register-unsafely-without-email" ;;
  *) email_arg="--email $email" ;;
esac

# Enable staging mode if needed
if [ $staging != "0" ]; then staging_arg="--staging"; fi

docker-compose run --rm --entrypoint "\
  certbot certonly --webroot -w /var/www/certbot \
    $staging_arg \
    $email_arg \
    $domain_args \
    --rsa-key-size $rsa_key_size \
    --agree-tos \
    --force-renewal" certbot
echo

echo "### Reloading nginx ..."
docker-compose exec nginx nginx -s reload

