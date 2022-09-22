from ctypes import CDLL
import os

import wireguard_util


current_directory = os.path.dirname(os.path.realpath(__file__))
wireguard_lib = CDLL(current_directory + "/bin/wireguard.so")


def create_server(name, private_key, port=51280):
    """"""
    wireguard_lib.add_server_device(name, port, private_key)

    os.system("ip link set up {name}")


def add_server_peer(name, public_key, ip_address, port):
    """"""
    wireguard_lib.add_server_peer(name, public_key, ip_address, port)

def generate_key_pair():
    """Generate a pair of keys (private, public)

    Returns: A tuple of the (private_key, public_key)

    """
    private_key, public_key = wireguard_util.generate_empty_key_pair()
    # Generate private and public keys
    wireguard_lib.wg_generate_private_key(private_key)
    wireguard_lib.wg_generate_public_key(public_key, private_key)
    return private_key, public_key
