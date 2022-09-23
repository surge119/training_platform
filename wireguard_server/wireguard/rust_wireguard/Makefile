BIN = ./bin
TARGET = $(BIN)/rust_wireguard

$(TARGET): src/main.rs $(BIN)/libwireguard.a
	rustc $< -L $(BIN) -l static=wireguard

$(BIN)/libwireguard.a: $(BIN)/wireguard_rust.o $(BIN)/wireguard.o
	ar rcs $@ $^

$(BIN)/wireguard.o: src/wireguard.c src/wireguard.h
	gcc -c -o $@ $<

$(BIN)/wireguard_rust.o: src/wireguard_rust.c src/wireguard.c src/wireguard.h
	gcc -c -o $@ $<

clean:
	rm -f $(BIN)/*.o $(BIN)/*.a $(TARGET)
