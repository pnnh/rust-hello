all : build

build :
	cargo build --color=always --package rust_hello --bin rust_hello

clean :
	rm -rf ./target