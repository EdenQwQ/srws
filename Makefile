build: src/main.rs Cargo.toml
	cargo build --release

install: target/release/srws
	@mkdir -p /usr/bin
	@mv target/release/srws /usr/bin/srws

uninstall:
	@rm /usr/bin/srws
