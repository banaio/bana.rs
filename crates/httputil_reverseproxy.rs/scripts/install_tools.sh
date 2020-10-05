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

# https://stackoverflow.com/questions/23356779/how-can-i-store-the-find-command-results-as-an-array-in-bash/54561526
TOOLS_FOUND=(
  $(
    find $(pwd)/scripts -mindepth 1 -maxdepth 1 \
      \( \
      -iname '**install_*.sh*' \
      \) \
      -a \
      \( -not -iname '*functions.sh*' \)
  )
)

# create a space delimited string from array
TOOLS_FOUND_SPACES_DELIMITED="${TOOLS_FOUND[*]}"
# use parameter expansion to substitute spaces with comma
TOOLS_LINES=${TOOLS_FOUND_SPACES_DELIMITED// /$'\n'}
TOOLS_TOTAL=$(echo "${TOOLS_LINES}" | wc -l)

print_separator_v3
printf -- "${INDENT}%b" \
  "TOOLS_FOUND=${TOOLS_FOUND[*]}" $'\n' \
  "TOOLS_FOUND_SPACES_DELIMITED=${TOOLS_FOUND_SPACES_DELIMITED}" $'\n' \
  "TOOLS_TOTAL=${TOOLS_TOTAL}" $'\n'
# "TOOLS_LINES=${TOOLS_LINES}" $'\n' \

print_separator

for TOOL in "${TOOLS_FOUND[@]}"; do
  echo "  TOOL='${TOOL}'"
done

print_separator

# for x in "${!TOOLS_FOUND[@]}"; do
#   TOOL="${TOOLS_FOUND[$x]}"
#   echo "  TOOL='${TOOL}'"
#   # code --new-window "${PROJECT}"
# done

# printf "%b" \
#   "${RED_STRING}" \
#   "TODO" \
#   $'\n'
# exit 1
