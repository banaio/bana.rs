#!/usr/bin/env bash
set -euf \
  -o nounset \
  -o errexit \
  -o noclobber \
  -o pipefail
shopt -s \
  extglob \
  globstar \
  nullglob

SCRIPT_NAME="$(basename "$0")"
UNAME_S=$(uname -s)
PWD="$(pwd)"

INDENT="  "
RESET="$(tput sgr0)"
BOLD="$(tput bold)"
RED="$(
  tput bold
  tput setaf 1
)"
RED_STRING="${RED}${SCRIPT_NAME} - ERROR: ${RESET}"
YELLOW="$(
  tput bold
  tput setaf 3
)"
YELLOW_STRING="${YELLOW}${SCRIPT_NAME} - WARN: ${RESET}"
GREEN="$(
  tput bold
  tput setaf 2
)"
GREEN_STRING="${GREEN}${SCRIPT_NAME} - INFO: ${RESET}"

function print_separator() {
  printf -- '-%.0b' $(seq 1 $(($(tput cols) - 1))) $'\n'
}

function print_separator_v2() {
  # https://unix.stackexchange.com/questions/352866/convert-arg-to-uppercase-to-pass-as-variable
  # | tr [a-z] [A-Z]
  # | tr '[:lower:]' '[:upper:]')
  # newvarname=${3^^}
  # local header="${1^^}" # uppercased
  local header="${1}"
  local padding="${2}"
  local separator_chars=""
  separator_chars=$(printf -- '-%.0s' $(seq 1 $(($(tput cols) - ${#header} - ${#padding}))))
  printf "%b" "$(
    tput bold
    tput setaf 2
  )" "${header}" "${padding}" "${separator_chars}" "$(tput sgr0)" "\n"
}

function print_separator_v3() {
  print_separator_v2 "${SCRIPT_NAME}" ":  "
}

function print_env() {
  print_separator_v3
  printf '%b' \
    "functions.sh: " $'\n' \
    "\t${GREEN}SCRIPT_NAME=${RESET}${SCRIPT_NAME}" $'\n' \
    "\t${GREEN}PWD=${RESET}${PWD}" $'\n' \
    "\t${GREEN}UNAME_S=${RESET}${UNAME_S}" $'\n'
}

export SCRIPT_NAME
export UNAME_S

export INDENT
export RESET
export BOLD
export RED
export RED_STRING
export YELLOW
export YELLOW_STRING
export GREEN
export GREEN_STRING

export ERROR_STRING="${RED_STRING}"
export WARN_STRING="${YELLOW_STRING}"
export INFO_STRING="${GREEN_STRING}"

# print_env
