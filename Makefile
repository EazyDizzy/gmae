run:
	cargo run --package gmae --bin gmae --release -vv
run-debug-schedule:
	cargo run --package gmae --bin gmae --release | dot -Tsvg > debug/schedule.svg

clean-old:
	cargo sweep -t 10
	cargo sweep --toolchains="nightly"