all : build

build :
	cargo build --color=always --message-format=json-diagnostic-rendered-ansi --package rust_hello --bin rust_hello

clean :
	rm -rf ./target