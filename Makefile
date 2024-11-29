all: build doc copy
all-dev: build-dev doc copy-dev

build:
	cargo build --release

build-dev:
	cargo build

doc:
	cargo doc

copy:
	cp target/release/minigrep .

copy-dev:
	cp target/debug/minigrep .

clean:
	cargo clean
	rm minigrep