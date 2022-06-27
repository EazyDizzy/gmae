run:
	cargo run --package gmae --bin gmae --release -vv

clean-old:
	cargo sweep -t 10
	cargo sweep --toolchains="nightly"