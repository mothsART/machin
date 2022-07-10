# Install to /usr unless otherwise specified, such as `make PREFIX=/app`
PREFIX=/usr

# What to run to install various files
INSTALL=install -D
# Run to install the actual binary
INSTALL_PROGRAM=$(INSTALL)
# Run to install application data, with differing permissions
INSTALL_DATA=$(INSTALL) -m 644

# Directories into which to install the various files
bindir=$(DESTDIR)$(PREFIX)/bin
sharedir=$(DESTDIR)$(PREFIX)/share

.PHONY: clean clean-all install cargo-publish push

# Build the application
target/release/machin: src
	cargo build --release

test:
	cargo test

install: target/release/machin copy-data
	# Install binary
	$(INSTALL_PROGRAM) target/release/machin $(bindir)/machin

cargo-publish:
	cargo clippy && cargo fmt && cargo publish --no-verify

clean:
	cargo clean

push:
	cargo clippy && cargo fmt && git push
