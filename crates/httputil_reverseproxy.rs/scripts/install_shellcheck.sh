#!/usr/bin/env bash
# Installs:
# *
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

CMD="shellcheck"

print_separator_v3
printf -- "${INDENT}%b" \
  "CMD=${CMD}" $'\n'

printf "%b" \
  "${RED_STRING}" \
  "install shellcheck, shellcheck=$(command -v "${CMD}")" \
  $'\n'
exit 0
