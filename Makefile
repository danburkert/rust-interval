RUSTC ?= rustc
RUSTC_FLAGS ?=

SRC = $(shell find src -name '*.rs')

all: libinterval

libinterval: $(SRC)
		mkdir -p target
		$(RUSTC) --out-dir target src/lib.rs

test: $(SRC)
		mkdir -p target
		$(RUSTC) --test --out-dir target src/lib.rs
		./target/interval

clean:
		@rm -rf target

.PHONY: clean
