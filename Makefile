target buildmin:
	cargo build --all-targets
target run:
	rustc -o samuelville main.rs
target build:
	cargo build --all-targets
	cargo deb