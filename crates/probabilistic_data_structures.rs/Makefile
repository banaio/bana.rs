# https://gist.github.com/rsperl/d2dfe88a520968fbc1f49db0a29345b9
# http://www.lunderberg.com/2015/08/25/cpp-makefile-pretty-output/
# http://agdr.org/2020/05/14/Polyglot-Makefiles.html
# https://tech.davis-hansson.com/p/make/

# FLAGS_SHELL												:= -eufxv
FLAGS_SHELL												:= -euf
# .ONESHELL:
SHELL															:= bash
.SHELLFLAGS 											:= ${FLAGS_SHELL} \
	-o nounset \
	-o errexit \
	-o noclobber \
	-o pipefail \
	-c
# IFS														:= $'\n\t'

# .DEFAULT_GOAL										:= all
# .DELETE_ON_ERROR:
MAKEFLAGS 												+= --warn-undefined-variables --environment-overrides #--print-directory #--no-builtin-rules #--no-builtin-variables
# MAKEFLAGS 											+= --jobs=32 --output-sync=target # https://www.gnu.org/software/make/manual/html_node/Parallel-Output.html#Parallel-Output
# MAKEFLAGS 											+= --just-print --dry-run --recon # debug

RESET															:= $(shell tput sgr0)
BOLD   														:= $(shell tput bold)
RED																:= $(shell tput bold; tput setaf 1)
GREEN															:= $(shell tput bold; tput setaf 2)
YELLOW														:= $(shell tput bold; tput setaf 3)

DIR_NAME													:= $(shell basename $(shell pwd))
PATH															:= $(shell echo "${HOME}/.cargo/bin:${PATH}")
PATH_FULL													:= $(shell pwd)

PKG_MANAGER_INSTALL_COMMAND				:= DEBIAN_FRONTEND="noninteractive" sudo apt-get -qq install -y --yes --assume-yes -qq

export TERM												?= xterm-256color
export VERBOSE										?= --verbose
export CARGO_TERM_COLOR						?= always
export CARGO_TERM_VERBOSE					?= 1
# export CARGO_INCREMENTAL					?= 0
# export RUSTFLAGS									?= "-Zprofile -Ccodegen-units=1 -Cinline-threshold=0 -Clink-dead-code -Coverflow-checks=off -Cpanic=abort -Zpanic_abort_tests"
# export SHELLCHECK_OPTS						?= --shell=bash --check-sourced --external-sources

define print_separator
@printf "%b" "${GREEN}" "$@:" `` "  " `` `printf -- '-%.0s' $(shell seq 1 $$(expr $$(tput cols) - `expr length "$@"` - 3))` "${RESET}" $$'\n'
endef

# This allows us to accept extra arguments (by doing nothing when we get a job that doesn't match, rather than throwing an error).
%:
	@:

# .PHONY: format run
.PHONY: all
all: format format_fix run

.PHONY: debug_env
debug_env:
	$(print_separator)
	@printf '%b' \
		"${GREEN}PATH=${RESET}${PATH}" $$'\n' \
		"${GREEN}DIR_NAME=${RESET}${DIR_NAME}" $$'\n' \
		"${GREEN}VERBOSE=${RESET}${VERBOSE}" $$'\n' \
		"${GREEN}CARGO_TERM_COLOR=${RESET}${CARGO_TERM_COLOR}" $$'\n' \
		"${GREEN}CARGO_TERM_VERBOSE=${RESET}${CARGO_TERM_VERBOSE}" $$'\n'

#@echo -e $(foreach VAR,$(.VARIABLES),"${BOLD}${MISC}$(VAR)${RESET}=$($(VAR))\n")

.PHONY: init
init: install_tools
	$(print_separator)

.PHONY: cargo_publish
cargo_publish: GIT_REV=$(shell git rev-parse --short HEAD)
cargo_publish:
	$(print_separator)
	$(error TODO)

.PHONY: clean
clean:
	$(print_separator)
	cargo ${VERBOSE} clean

.PHONY: build
build:
	$(print_separator)
	@#RUSTFLAGS="-Z avoid-dev-deps -Z unstable-options -Z timings"
	cargo ${VERBOSE} build --workspace --release --jobs=32 --all-features

.PHONY: test
test:
	$(print_separator)
	@#RUSTFLAGS="-Z unstable-options"
	@#BUILD_TYPE="--release"
	BUILD_TYPE=""
	cargo ${VERBOSE} test --workspace ${BUILD_TYPE} --no-fail-fast -- --test-threads 32 --nocapture --show-output --format pretty

# https://stackoverflow.com/questions/6273608/how-to-pass-argument-to-makefile-from-command-line/6273809
# https://stackoverflow.com/questions/2214575/passing-arguments-to-make-run
#args = `arg="$(filter-out $@,$(MAKECMDGOALS))" && echo $${arg:-${1}}`
#echo $(call args,defaultstring)
.PHONY: run
run:
	$(print_separator)
	cargo ${VERBOSE} run -- $(filter-out $@,$(MAKECMDGOALS))

.PHONY: run_test
run_test:
	$(print_separator)
	cargo ${VERBOSE} run -- $(filter-out $@,$(MAKECMDGOALS))

.PHONY: lint
lint:
	$(print_separator)
	cargo ${VERBOSE} clippy --

.PHONY: format_fix
format_fix:
	$(print_separator)
	cargo ${VERBOSE} fix ${VERBOSE} --allow-dirty --allow-staged --workspace --jobs=32 --all-features
	cargo ${VERBOSE} fix ${VERBOSE} --allow-dirty --allow-staged --workspace --jobs=32 --all-features --profile='test'

.PHONY: format
format:
	$(print_separator)
	cargo ${VERBOSE} fmt ${VERBOSE} --all -- ${VERBOSE}

.PHONY: format_nightly
format_nightly:
	$(print_separator)
	cargo ${VERBOSE} +nightly fmt ${VERBOSE} --all -- --unstable-features

# VERBOSE="${VERBOSE}" cargo +nightly --all
# #rustup toolchain install nightly
# rustup toolchain install --profile minimal --component rustfmt,clippy,cargo nightly
# rustup component add rustfmt --toolchain nightly && rustup component add cargo --toolchain nightly

.PHONY: rustup
rustup:
	$(print_separator)
	${PKG_MANAGER_INSTALL_COMMAND} \
		parallel \
		libssl-dev \
		libssh2-devel \
		openssl-devel \
		libgit2-devel || (exit_code=$$?; printf "%b" "${RED}ERROR: ${RESET}" "failed to install - PACKAGES='parallel, libssl-dev, libssh2-devel, openssl-devel, libgit2-dev', exit_code=$${exit_code}" $$'\n'; exit $${exit_code})
	rustup ${VERBOSE} show
	rustup ${VERBOSE} update
	rustup ${VERBOSE} component add clippy
	rustup ${VERBOSE} component add rustfmt
	rustup ${VERBOSE} component add rustfmt-preview
	@# #rustup toolchain install nightly
	rustup +nightly ${VERBOSE} toolchain install --profile minimal --component rustfmt,clippy,cargo nightly
	rustup +nightly ${VERBOSE} component add rustfmt --toolchain nightly
	rustup +nightly ${VERBOSE} component add cargo --toolchain nightly

# 2>&1 || printf "%b" "${ERROR_STRING}" "failed to install - COMMAND=${COMMAND}, exit_code=$?" $'\n'

.PHONY: cargo_update_all
cargo_update_all:
	$(print_separator)
	# A cargo subcommand for checking and applying updates to installed executables
	${PKG_MANAGER_INSTALL_COMMAND} \
		parallel \
		libssl-dev \
		libssh-dev \
		libgit2-dev || (exit_code=$$?; printf "%b" "${RED}ERROR: ${RESET}" "failed to install - PACKAGES='parallel, libssl-dev, libssh-dev, libgit2-dev', exit_code=$${exit_code}" $$'\n'; exit $${exit_code})
	cargo ${VERBOSE} install cargo-edit
	cargo ${VERBOSE} upgrade --workspace
	# cargo ${VERBOSE} install cargo-update
	# cargo ${VERBOSE} install-update -a

# 2>&1 || printf "%b" "${ERROR_STRING}" "failed to install - COMMAND=${COMMAND}, exit_code=$?" $'\n'

.PHONY: generate_random
generate_random:
	$(print_separator)
	./scripts/generate_random.sh

.PHONY: install_tools
install_tools: install_lld install_shfmt install_shellcheck install_yamllint
	$(print_separator)
	@printf '%b' \
		"${GREEN}*lld*|*ld-10*=${RESET}$(shell find /usr/bin -iname '*lld*' -o -iname '*ld-10*')" $$'\n' \
		"${GREEN}shfmt=${RESET}$(shell which shfmt)" $$'\n' \
		"${GREEN}shellcheck=${RESET}$(shell which shellcheck)" $$'\n' \
		"${GREEN}yamllint=${RESET}$(shell which yamllint)" $$'\n'

.PHONY: install_lld
install_lld:
	$(print_separator)
	./scripts/install_lld.sh

.PHONY: install_shfmt
install_shfmt:
	$(print_separator)
	./scripts/install_shfmt.sh

.PHONY: install_shellcheck
install_shellcheck:
	$(print_separator)
	./scripts/install_shellcheck.sh

.PHONY: install_yamllint
install_yamllint:
	$(print_separator)
	./scripts/install_yamllint.sh

.PHONY: web
web:
	$(print_separator)
	$(error TODO)

# $(print_separator)
# $(call print_separator)

# have_term := $(shell echo $$TERM)
# ifdef have_term
# define my_color =
#     @tput setaf $2
#     @tput bold
#     @echo $1
#     @tput sgr0
# endef
# else
# my_color = @echo $1
# endif

# all:
#     $(call my_color, "Hello world", 2)
