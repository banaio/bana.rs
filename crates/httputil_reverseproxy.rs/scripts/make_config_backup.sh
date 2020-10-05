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
print_env

backup_folder_name="$(date --utc +'backup_%d_%B_%Y_%H-%M-%S_%Z')"
backup_folder_path="./.configs/${backup_folder_name}"

print_separator_v3
printf -- "${INDENT}%b" \
  "backup_folder_name=${backup_folder_name}" $'\n' \
  "backup_folder_path=${backup_folder_path}" $'\n'

mkdir -v -p "${backup_folder_path}"

printf "%b" \
  "${GREEN_STRING}" \
  "created backup_folder_path=${backup_folder_path}" \
  $'\n'

cd "${backup_folder_path}"
pwd

exit 0
