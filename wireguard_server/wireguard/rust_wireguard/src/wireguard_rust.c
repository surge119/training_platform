#include <stdio.h>
#include <stdlib.h>
#include "wireguard.h"

void generate_key() {
    wg_key private_key;
    wg_generate_private_key(private_key);
    printf("Key: %s\n", private_key);
    printf("Size: %d\n", strlen(private_key));
}

//int main() {
//    printf("TEST: %s\n", generate_key());
//}
