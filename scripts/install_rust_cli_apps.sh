#!/usr/bin/env bash
# Usage: install_rust_cli_apps.sh
#
# Install Rust CLI apps
set -euf \
  -o nounset \
  -o errexit \
  -o noclobber \
  -o pipefail
shopt -s \
  extglob \
  globstar \
  nullglob

UNAME_S="$(uname -s)"

if [[ "${UNAME_S}" = "Linux" ]]; then
  set -euf \
    -o nounset \
    -o errexit \
    -o errtrace \
    -o noclobber \
    -o pipefail \
    -o posix \
    -o functrace \
    -o notify

  shopt -s \
    extglob \
    globstar \
    nullglob \
    failglob \
    gnu_errfmt \
    localvar_unset \
    dotglob \
    xpg_echo

  function print_separators() {
    local sep_char="${1:-}"
    printf -- "${sep_char}%.0s" $(seq 1 $(($(tput cols) - ${#sep_char} - 1))) $'\n'
  }
elif [[ "${UNAME_S}" = "Darwin" ]]; then
  # set -euf \
  #   -o nounset \
  #   -o errexit \
  #   -o noclobber \
  #   -o pipefail
  # shopt -s \
  #   extglob \
  #   globstar \
  #   nullglob

  function print_separators() {
    local sep_char="${1:-}"
    printf -- "${sep_char}%.0s" $(seq 1 $(($(tput cols) - ${#sep_char})))
    echo
  }
else
  printf '%b' "$(tput bold)" "$(tput setaf 1)" "ERROR: " "$(tput sgr0)" "unsupported platform" "\n"
  exit 1
fi

print_separators

PATH="$HOME/.cargo:${PATH}"
APPS=(
  'gitui'
  'bottom'
  'exa'
  'bat'
  'ripgrep'
  'fd'
  'tldr'
)
TOTAL_APPS=$(printf -- '%s\n' "${APPS[@]}" | wc -l)
export PATH
export APPS
export TOTAL_APPS

printf -- "%b" \
  "APPS=" "$(tput bold)" "$(tput setaf 2)" "${APPS[*]}" "$(tput sgr0)" $'\n' \
  "TOTAL_APPS=" "$(tput bold)" "$(tput setaf 2)" "${TOTAL_APPS}" "$(tput sgr0)" $'\n'

# TODO: Use array
function install_all() {
  print_separators ">"
  # https://github.com/extrawurst/gitui
  cargo install gitui
  print_separators
  # https://github.com/clementtsang/bottom
  # OR, --locked may be required due to how cargo install works
  cargo install bottom || cargo install bottom --locked
  print_separators
  # https://github.com/ogham/exa
  cargo install exa
  # cargo install --no-default-features exa
  print_separators
  # https://github.com/sharkdp/bat
  cargo install --locked bat
  print_separators
  # https://github.com/BurntSushi/ripgrep/
  cargo install ripgrep
  print_separators
  # https://github.com/sharkdp/fd
  cargo install fd-find
  print_separators
  # https://github.com/tldr-pages/tldr
  # https://github.com/dbrgn/tealdeer
  cargo install tealdeer
  print_separators "<"
}

function test_all() {
  print_separators "."
  for APP in "${APPS[@]}"; do
    printf "%b" "APP=" "$(tput bold)" "$(tput setaf 2)" "${APP}" "$(tput sgr0)" $'\n'
    "${APP}" --version || true
    print_separators "."
  done
}

# install_all
test_all
