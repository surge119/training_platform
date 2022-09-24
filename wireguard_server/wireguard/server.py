from python_wireguard import Key, Server, ClientConnection


def setup_vpn_server():
    """"""
    private, public = Key.key_pair()

    server = Server("wg-srv", private, "10.10.10.1/24", 51280)
    server.enable()
    return server


def client_test(server):
    private, public = Key.key_pair()
    client_ip = "10.10.10.2"
    client = ClientConnection(public, client_ip)
    print(private)

    server.add_client(client)


if __name__ == "__main__":
    server = setup_vpn_server()
    client_test(server)
