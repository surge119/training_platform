BIN = ./bin
TARGET = $(BIN)/rust_wireguard

VENDOR = src/vendor
WIREGUARD_SRC = $(VENDOR)/wireguard/wireguard.c $(VENDOR)/wireguard/wireguard.h
WIREGUARD_RUST_SRC = src/wireguard_rust.c $(WIREGUARD_SRC)

$(TARGET): src/main.rs $(BIN)/libwireguard.a
	rustc $< -L $(BIN) -l static=wireguard

$(BIN)/libwireguard.a: $(BIN)/wireguard_rust.o $(BIN)/wireguard.o
	ar rcs $@ $^

$(BIN)/wireguard.o: $(WIREGUARD_SRC)
	gcc -c -o $@ $<

$(BIN)/wireguard_rust.o: $(WIREGUARD_RUST_SRC)
	gcc -c -o $@ $<

clean:
	rm -f $(BIN)/*.o $(BIN)/*.a $(TARGET)
