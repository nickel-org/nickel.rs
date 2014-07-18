LIBS=-L target/deps

.PHONY: deps examples

all: deps examples

deps:
	cargo build -v

examples: deps
	cp target/lib* examples/lib/
	cp target/deps/* examples/lib/
	rustc -L examples/lib examples/example.rs -o examples/example

doc: deps
	rustdoc $(LIBS) src/lib.rs

clean:
	cargo clean
	rm examples/lib/*

run: 
	./examples/example

buildrun: examples run

