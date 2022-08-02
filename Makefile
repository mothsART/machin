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

.PHONY: clean clean-all install copy-data uninstall cargo-publish push

# Build the application
target/release/machin: src
	cargo build --release

test:
	cargo test

install: target/release/machin copy-data
	# Install binary
	$(INSTALL_PROGRAM) target/release/machin $(bindir)/machconvert
	$(INSTALL_PROGRAM) target/release/machin $(bindir)/machmap
	$(INSTALL_PROGRAM) target/release/machin $(bindir)/machreduce

copy-data:
	$(INSTALL_DATA) machconvert.bash $(sharedir)/bash-completion/completions/machconvert.bash
	$(INSTALL_DATA) machconvert.fish $(sharedir)/fish/vendor_completions.d/machconvert.fish
	$(INSTALL_DATA) _machconvert $(sharedir)/zsh/site-functions/_machconvert

	$(INSTALL_DATA) machmap.bash $(sharedir)/bash-completion/completions/machmap.bash
	$(INSTALL_DATA) machmap.fish $(sharedir)/fish/vendor_completions.d/machmap.fish
	$(INSTALL_DATA) _machmap $(sharedir)/zsh/site-functions/_machmap

	$(INSTALL_DATA) mapreduce.bash $(sharedir)/bash-completion/completions/mapreduce.bash
	$(INSTALL_DATA) mapreduce.fish $(sharedir)/fish/vendor_completions.d/mapreduce.fish
	$(INSTALL_DATA) _mapreduce $(sharedir)/zsh/site-functions/_mapreduce

uninstall:
	rm -f $(sharedir)/bash-completion/completions/machconvert.bash
	rm -f $(sharedir)/fish/vendor_completions.d/machconvert.fish
	rm -f $(sharedir)/zsh/site-functions/_machconvert

	rm -f $(sharedir)/bash-completion/completions/machmap.bash
	rm -f $(sharedir)/fish/vendor_completions.d/machmap.fish
	rm -f $(sharedir)/zsh/site-functions/_machmap

	rm -f $(sharedir)/bash-completion/completions/mapreduce.bash
	rm -f $(sharedir)/fish/vendor_completions.d/machconvert.fish
	rm -f $(sharedir)/zsh/site-functions/_machconvert

	# Remove the binary
	rm -f $(bindir)/machconvert
	rm -f $(bindir)/machmap
	rm -f $(bindir)/machreduce

cargo-publish:
	cargo clippy && cargo fmt && cargo publish --no-verify

clean:
	cargo clean

push:
	cargo clippy && cargo fmt && git push
