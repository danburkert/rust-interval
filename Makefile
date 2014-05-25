RUSTC ?= rustc
RUSTC_FLAGS ?=

SRC = $(shell find src -name '*.rs')
QUICKCHECK_DIR := $(CURDIR)/lib/quickcheck

all: interval

interval: $(SRC) lib-quickcheck
		mkdir -p target
		$(RUSTC) -L $(QUICKCHECK_DIR) --out-dir target src/lib.rs

test: $(SRC)
		mkdir -p target
		$(RUSTC) --test --out-dir target src/lib.rs
		./target/interval

.PHONY: lib-quickcheck
lib-quickcheck:
		make -C $(QUICKCHECK_DIR)

.PHONY: clean
clean:
		make clean -C $(QUICKCHECK_DIR)
		@rm -rf target
