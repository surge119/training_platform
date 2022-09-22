from flask import Flask

import wireguard_interface


def start_vpn_server():
    """"""
    private_key, public_key = wireguard_interface.generate_key_pair()
    wireguard_interface.create_server("platform_vpn", private_key)


def add_server_peer():
    """"""


def start_api():
    """"""
    app = Flask(__name__)

    @app.route('/vpn/generate_peer')
    def generate_peer():
        """"""

    app.run(host="0.0.0.0", port=80)


if __name__ == "__main__":
    start_vpn_server()

