#!/usr/bin/env bash
# Installs:
# * https://github.com/mvdan/sh
# * https://github.com/mvdan/sh/releases
# * https://github.com/mvdan/sh/releases/tag/v3.1.2
set -euf \
  -o nounset \
  -o errexit \
  -o noclobber \
  -o pipefail
shopt -s \
  extglob \
  globstar \
  nullglob

source ./scripts/functions.sh

GOPATH="$(go env GOPATH)"
GO_BIN="${GOPATH}/bin"
GO_BIN_PATH="${GOPATH}/bin"
PATH_OLD="${PATH}"
PATH="${GOPATH}/bin:${PATH}"
COMMAND="shfmt"
COMMAND_VERSION="v3.1.2"
COMMAND_DOWNLOAD="mvdan.cc/sh/v3/cmd/shfmt@${COMMAND_VERSION}"
TARGET="${GOPATH}/bin/shfmt"
LINK="/usr/local/bin/shfmt"

print_separator_v3
printf -- "${INDENT}%b" \
  "GO_BIN=${GO_BIN}" $'\n' \
  "GO_BIN_PATH=${GO_BIN_PATH}" $'\n' \
  "PATH_OLD=${PATH_OLD}" $'\n' \
  "PATH=${PATH}" $'\n' \
  "COMMAND=${COMMAND}" $'\n' \
  "COMMAND_VERSION=${COMMAND_VERSION}" $'\n' \
  "COMMAND_DOWNLOAD=${COMMAND_DOWNLOAD}" $'\n' \
  "TARGET=${TARGET}" $'\n' \
  "LINK=${LINK}" $'\n'

function print_error_and_exit() {
  local exit_code="$?"
  # printf "%b" \
  #   "${RED_STRING}" \
  #   "shfmt (https://github.com/mvdan/sh) not found, 'shfmt -version'=$(shfmt -version), exit_code=${exit_code}" \
  #   $'\n'
  printf "%b" \
    "${RED_STRING}" "${COMMAND} (https://github.com/mvdan/sh) not found" $'\n'
  printf -- "${INDENT}%b" \
    "COMMAND_DOWNLOAD=${COMMAND_DOWNLOAD}" $'\n' \
    "exit_code=${exit_code}" $'\n' \
    "shfmt -version=$(shfmt -version)" $'\n'
  exit 1
}

print_separator_v2 "${SCRIPT_NAME}" ": old shfmt (https://github.com/mvdan/sh), version=$("${COMMAND}" -version), path=$(command -v "${COMMAND}"), COMMAND_DOWNLOAD=${COMMAND_DOWNLOAD}"

cd "${GOPATH}"

GO111MODULE=on go get -v "${COMMAND_DOWNLOAD}" || (
  printf "%b" \
    "${RED_STRING}" \
    "shfmt (https://github.com/mvdan/sh) failed to install, COMMAND_DOWNLOAD=${COMMAND_DOWNLOAD}, exit_code=${?}" \
    $'\n'
  exit 1
)
# GO111MODULE=on go get mvdan.cc/sh/v3/cmd/shfmt

print_separator
shfmt 1>/dev/null 2>&1 -version || "${GO_BIN}"/shfmt 1>/dev/null 2>&1 2>&1 -version || print_error_and_exit "$?"

printf "%b" \
  "${GREEN_STRING}" "installed ${COMMAND} - shfmt (https://github.com/mvdan/sh)" $'\n'
printf -- "${INDENT}%b" \
  "version=$("${COMMAND}" -version)" $'\n' \
  "path=$(command -v "${COMMAND}")" $'\n' \
  "COMMAND_DOWNLOAD=${COMMAND_DOWNLOAD}" $'\n'

print_separator
ls -lah /usr/local/bin/shfmt || printf "%b" "${YELLOW_STRING}" "${LINK} does not exist" $'\n'

if [[ "${UNAME_S}" = "Linux" ]] || [[ "${UNAME_S}" = "Darwin" ]]; then
  if [[ ! -x "$(command -v "${LINK}" 2>/dev/null)" ]]; then
    # --force
    sudo ln --symbolic --verbose "${TARGET}" "${LINK}"
  else
    # sudo ln --force --symbolic --verbose "${TARGET}" "${LINK}"
    sudo ln --symbolic --verbose "${TARGET}" "${LINK}" || (
      printf "%b" \
        "${YELLOW_STRING}" "symlink already exists" $'\n'
      printf -- "${INDENT}%b" \
        "LINK=${LINK}" $'\n' \
        "TARGET=${TARGET}" $'\n' \
        "details=$(ls -lah "$(command -v "${COMMAND}")")" $'\n'
    )
  fi
else
  printf "%b" \
    "${RED_STRING}" \
    "symlink cannot be created to LINK=${LINK}, TARGET=${TARGET} - Unsupported OS, UNAME_S=${UNAME_S}" \
    $'\n'
  exit 1
fi

ls -lah /usr/local/bin/shfmt || printf "%b" "${YELLOW_STRING}" "LINK=${LINK} does not exist" $'\n'
print_separator
/usr/local/bin/shfmt 1>/dev/null 2>&1 -version || "${GO_BIN}"/shfmt 1>/dev/null 2>&1 2>&1 -version || print_error_and_exit "$?"

exit 0
