all: wireguard

python_wireguard/bin/wireguard.so: src/bin/wireguard.o src/bin/wireguard_python.o
	gcc -Wall -fPIC -shared src/bin/wireguard.o src/bin/wireguard_python.o

src/bin/wireguard.o: src/wireguard.c src/wireguard.h
	gcc -c src/wireguard.c src/wireguard.h

src/bin/wireguard_python.o: src/wireguard_python.c
	gcc -c src/wireguard_python.c

clean:
	rm -f src/bin/*.o python_wireguard/bin/*.so
