# The Minilux Programming Language Makefile
all:
	cargo build --release
	cp target/release/minilux ./minilux

# MAC USERS: Edit the install path to /usr/local/bin/minilux instead
# As you won't be able to install directly into /usr/bin without major tinkering.
# - Alexia.

install:
	cargo build --release
	sudo cp minilux /usr/bin/minilux
	sudo chmod 755 /usr/bin/minilux

uninstall:
	sudo rm -f /usr/bin/minilux

clean:
	cargo clean
	rm -f minilux
