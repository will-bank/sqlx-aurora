.PHONY: setup up install_binstall install_watch install_nextest install_sqlx test_oneshot test run fmt clippy clippy_check stack_up wait_stack stack_down stop destroy sqlx_prepare migrate migrate_add psql

setup:
	@cp .github/pre-commit .git/hooks/
	@cargo --version >/dev/null || curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh && source "${HOME}/.cargo/env"
	@rustup default 1.81 >/dev/null
	@$(MAKE) install_watch
	@$(MAKE) install_sqlx
	@echo "\nSetup completed! Remember to add the following line to your ~/.zshrc or ~/.bashrc:\n    source \"$\{HOME}/.cargo/env\""

install_watch:
	@cargo binstall -y -q cargo-watch

test:
	@cargo watch -q -c -x 'nextest run ${FILTER} --no-capture'

fmt:
	cargo fmt --all

clippy:
	cargo clippy --fix --all-targets --all-features --allow-staged --allow-dirty -- -Dwarnings -Dclippy::unwrap_used

clippy_check:
	cargo clippy --all-targets --all-features -- -Dwarnings -Dclippy::unwrap_used

