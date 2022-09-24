#include <stdio.h>
#include <stdlib.h>

#include "vendor/wireguard/wireguard.h"

void test1(char* str) {
    printf("Char: %s\n", str);
}

char* test() {
    char *str = malloc(15);
    strncpy(str, "Hello, World\0", 15);
    return str;
}

char* base64_private_key() {
    // Declare private key and base64 private key
    wg_key private_key;
    wg_key_b64_string *enc_private_key = malloc(sizeof(wg_key_b64_string));

    wg_generate_private_key(private_key);

    wg_key_to_base64(enc_private_key, private_key);

    printf("Size: %d\n", strlen(enc_private_key));
    printf("CKey: %s\n", enc_private_key);

    return enc_private_key;
}

char* generate_private_key() {
    wg_key *private_key = malloc(sizeof(wg_key));
    wg_generate_private_key(private_key);
    printf("CKey: %s\n", private_key);

    return private_key;
}

void generate_key() {
    wg_key private_key;
    wg_generate_private_key(private_key);
    test(private_key);
}

/**
 * @brief Add a server to wireguard
 *
 * @param device_name The name of the new server interface
 * @param port The port to listen on
 * @param private_key The private key of the server
 */
void add_server_device(char *device_name, uint16_t port, wg_key private_key)
{
    wg_device new_device = {
        .flags = WGDEVICE_HAS_PRIVATE_KEY | WGDEVICE_HAS_LISTEN_PORT,
        .listen_port = port,
    };

    snprintf(new_device.name, IFNAMSIZ, "%s", device_name);
    memcpy(new_device.private_key, private_key, sizeof(new_device.private_key));

    if (wg_add_device(new_device.name) < 0)
    {
        perror("Unable to add device");
        exit(1);
    }

    if (wg_set_device(&new_device) < 0)
    {
        perror("Unable to set device");
        exit(1);
    }
}

/**
 * Add a peer to device 'device_name'.
 * @param device_name
 * @param public_key
 * @param ip_address
 */
void add_server_peer(char *device_name, unsigned char *public_key, char *ip_address, uint16_t port)
{
    struct sockaddr_in dest_addr;
    bzero(&dest_addr, sizeof(dest_addr));
    dest_addr.sin_family = AF_INET;
    dest_addr.sin_port = htons(port);
    inet_pton(AF_INET, ip_address, &dest_addr.sin_addr);

    wg_allowedip allowed_ip;
    bzero(&allowed_ip, sizeof(allowed_ip));
    inet_pton(AF_INET, "0.0.0.0", &allowed_ip.ip4);
    allowed_ip.family = AF_INET;
    allowed_ip.cidr = 0;

    wg_peer new_peer = {
        .flags = WGPEER_HAS_PUBLIC_KEY | WGPEER_REPLACE_ALLOWEDIPS};

    new_peer.endpoint.addr4 = dest_addr;
    new_peer.first_allowedip = &allowed_ip;
    new_peer.last_allowedip = &allowed_ip;
    memcpy(new_peer.public_key, public_key, sizeof(new_peer.public_key));
    wg_device *device;
    if (wg_get_device(&device, device_name) < 0)
    {
        perror("Unable to get device");
        exit(1);
    }
    wg_peer *peer;
    if (device->last_peer == NULL)
    {
        device->first_peer = &new_peer;
        device->last_peer = &new_peer;
    }
    else
    {
        peer = device->last_peer;
        peer->next_peer = &new_peer;
        device->last_peer = &new_peer;
    }

    wg_set_device(device);
}
