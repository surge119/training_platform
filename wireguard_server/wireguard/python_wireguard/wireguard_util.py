from ctypes import create_string_buffer


def generate_empty_key_pair():
    """Generate a pair of keys

    Returns: A pair of empty keys

    """
    return generate_empty_key(), generate_empty_key()


def generate_empty_key():
    """Generate an empty string for keys

    Returns: An empty string of size 32 for private and public keys

    """
    return create_string_buffer(b"\000"*32)
