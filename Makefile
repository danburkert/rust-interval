RUSTC ?= rustc

TARGET = $(CURDIR)/target

INTERVAL_SRC = $(wildcard ${CURDIR}/src/*.rs ${CURDIR}/src/**/*.rs)
INTERVAL_MAIN = $(CURDIR)/src/lib.rs
INTERVAL_LIB = ${TARGET}/$(shell ${RUSTC} --crate-file-name ${INTERVAL_MAIN})
TEST_BIN = ${TARGET}/$(shell ${RUSTC} --test --crate-file-name ${INTERVAL_MAIN})

CHECK_SRC = $(wildcard ${CURDIR}/check/*.rs ${CURDIR}/check/**/*.rs)
CHECK_MAIN = $(CURDIR)/check/bin.rs
CHECK_BIN = ${TARGET}/$(shell ${RUSTC} --crate-file-name ${CHECK_MAIN})

QUICKCHECK_DIR := $(CURDIR)/lib/quickcheck
QUICKCHECK_SRC = $(wildcard ${QUICKCHECK_DIR}/src/*.rs ${QUICKCHECK_DIR}/src/**/*.rs)
QUICKCHECK_MAIN = ${QUICKCHECK_DIR}/src/lib.rs
QUICKCHECK_LIB = ${TARGET}/$(shell ${RUSTC} --crate-file-name ${QUICKCHECK_MAIN})

all: $(INTERVAL_LIB)

.PHONY: test
test: $(TEST_BIN)
	$(TEST_BIN)

.PHONY: check
check: $(CHECK_BIN)
	@RUST_LOG=quickcheck $(CHECK_BIN)

$(INTERVAL_LIB): $(QUICKCHECK_LIB) $(INTERVAL_SRC)
	@mkdir -p target
	$(RUSTC) -L $(TARGET) --out-dir ${TARGET} ${INTERVAL_MAIN}

$(TEST_BIN): $(INTERVAL_SRC)
	@mkdir -p target
	$(RUSTC) --test --out-dir ${TARGET} ${INTERVAL_MAIN}

$(QUICKCHECK_LIB): $(QUICKCHECK_SRC)
	@mkdir -p target
	$(RUSTC) -O --out-dir ${TARGET} $(QUICKCHECK_MAIN)

$(CHECK_BIN): $(INTERVAL_LIB) $(QUICKCHECK_LIB) $(CHECK_SRC)
	$(RUSTC) -L $(TARGET) --out-dir ${TARGET} ${CHECK_MAIN}

.PHONY: update
update:
	git submodule init
	git submodule update

.PHONY: clean
clean:
	@rm -rf target
