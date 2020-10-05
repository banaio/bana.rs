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

source ./scripts/functions.sh

PKGS="yamllint"
FORMAT="colored" #format="parsable" {parsable,standard,colored,auto}
CONFIG_DATA=".configs/.yamllint_extended.yml"
COMMAND="yamllint"

print_separator_v3
printf -- "${INDENT}%b" \
  "PKGS=${PKGS}" $'\n' \
  "FORMAT=${FORMAT}" $'\n' \
  "CONFIG_DATA=${CONFIG_DATA}" $'\n' \
  "COMMAND=${COMMAND}" $'\n'

if [[ "${UNAME_S}" = "Linux" ]]; then
  if [[ ! -x "$(command -v "${COMMAND}" 2>/dev/null)" ]]; then
    if ! dpkg -s "${PKGS}" >/dev/null 2>&1; then
      command=$(apt-cache search --names-only "${PKGS}" >/dev/null 2>&1)
      if [[ -n ${command} ]]; then
        echo "${INSTALL_COMMAND} ${PKGS}"
        # ${INSTALL_COMMAND} "${PKGS}"
      else
        printf "%b" \
          "${RED_STRING}" \
          "${PKGS} is not available for installs, UNAME_S=${UNAME_S}" \
          $'\n'
        exit 1
      fi
    fi
  else
    COMMAND_VERSION=$("${COMMAND}" 2>&1 --version | tr -d '\n' || true)
    printf "%b" \
      "${YELLOW_STRING}" \
      "COMMANDS - Command already exists, COMMAND=${COMMAND}, COMMAND_VERSION=${COMMAND_VERSION}" \
      $'\n'
  fi
elif [[ "${UNAME_S}" = "Darwin" ]]; then
  printf "%b" \
    "${RED_STRING}" \
    "INSTALL_COMMAND - Supported OS, UNAME_S=${UNAME_S}" \
    $'\n'
  exit 1
else
  printf "%b" \
    "${RED_STRING}" \
    "INSTALL_COMMAND - Unsupported OS, UNAME_S=${UNAME_S}" \
    $'\n'
  exit 1
fi

yamllint --format "${FORMAT}" --config-data "${CONFIG_DATA}" \
  .configs/.yamllint_extended.yml \
  .configs/.yamllint.yml \
  .github/workflows/ \
  _.github/workflows/

exit 0
