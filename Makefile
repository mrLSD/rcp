#
# Makefile
# @author Evgeny Ukhanov <mrlsd@ya.ru>
#

.PHONY: run, build, release, install

default: release

release:
	@echo Build release version...
	@cargo build --release
	@echo Done.

build:
	@echo Build debug version...
	@cargo build
	@echo Done.

install:
	@echo Install new Rust stable varsion...
	@curl -s https://static.rust-lang.org/rustup.sh | sudo sh
	@echo Installed:
	@rustc --version
	@echo Done.
