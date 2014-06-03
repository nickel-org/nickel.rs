LIBS=-L lib

.PHONY: clean floor deps examples

clean:
	rm -rf lib && mkdir lib

floor:
	rm -f lib/libfloor-*
	rustc $(LIBS) --opt-level=3 src/main.rs --out-dir lib/

deps:
	rm -f lib/libhttp*
	# this will soon be a git submodule
	cp /Applications/MAMP/htdocs/rust-http/build/libhttp* lib/

examples:
	rustc $(LIBS) examples/example.rs -o examples/example

all: clean deps floor examples

run:
	./examples/example