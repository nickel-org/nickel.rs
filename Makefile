LIBS=-L target/deps

.PHONY: deps examples

all: deps examples

deps:
	cargo build -v

examples: deps
	cargo test

doc: deps
	rustdoc $(LIBS) src/lib.rs

clean:
	cargo clean

run: 
	./target/test/example

buildrun: examples run

