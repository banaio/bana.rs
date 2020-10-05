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

COMMAND="shuf"
RANDOM_GENERATED_STRINGS_TOTAL="2"
OUTPUT_FILE_BASENAME="${PWD}/testdata/countdistinct"
OUTPUT_FILE_DIRNAME="$(dirname "${OUTPUT_FILE_BASENAME}")"
OUTPUT_FILES=$(echo "${OUTPUT_FILE_BASENAME}/elements"{,_binary,_hex}".txt")

print_separator_v3
echo "COMMAND=${COMMAND}"
echo "RANDOM_GENERATED_STRINGS_TOTAL=${RANDOM_GENERATED_STRINGS_TOTAL}"
echo "OUTPUT_FILE_BASENAME=${OUTPUT_FILE_BASENAME}"
echo "OUTPUT_FILE_DIRNAME=${OUTPUT_FILE_DIRNAME}"
echo "OUTPUT_FILES=${OUTPUT_FILES}"

if [[ ! -x "$(command -v "${COMMAND}" 2>/dev/null)" ]]; then
  printf "%b" \
    "${RED_STRING}" \
    "COMMAND - Command does not exist, COMMAND=${COMMAND}" \
    $'\n'
  exit 1
else
  # # COMMAND_VERSION="$(1>/dev/null 2>&1 ${COMMAND} --version | tr -d '\n')"
  # COMMAND_VERSION=$("${COMMAND}" 2>&1 --version | tr -d '\n' || true)
  printf "%b" \
    "${YELLOW_STRING}" \
    "COMMAND - Command exists, COMMAND=${COMMAND}" \
    $'\n'
fi

printf "%b" \
  "${GREEN_STRING}" \
  "generating random data" $'\n' \
  "\t${GREEN}OUTPUT_FILE_BASENAME${RESET}=${OUTPUT_FILE_BASENAME}" $'\n' \
  "\t${GREEN}OUTPUT_FILES${RESET}=${OUTPUT_FILES}" $'\n'

echo "OUTPUT_FILE_DIRNAME=${OUTPUT_FILE_DIRNAME}"
mkdir -p "${OUTPUT_FILE_DIRNAME}"

printf '%s' \
  "${GREEN}OUTPUT_FILE_DIRNAME${RESET}=" \
  "$(ls -1 "${OUTPUT_FILE_DIRNAME}")" \
  $'\n'

echo "OUTPUT_FILE_DIRNAME=${OUTPUT_FILE_DIRNAME}"
# /home/mbana/dev/bana-io/probabilistic_data_structures.rs/testdata/countdistinct/elements_binary.txt /home/mbana/dev/bana-io/probabilistic_data_structures.rs/testdata/countdistinct/elements_hex.txt
echo "OUTPUT_FILES=${OUTPUT_FILES}"

printf '%s' \
  "${GREEN}OUTPUT_FILE_BASENAME${RESET}=" \
  "$(ls -1C "${OUTPUT_FILE_BASENAME}")" \
  $'\n'

echo
# shellcheck disable=SC2086
rm --verbose ${OUTPUT_FILES} || printf "%b" \
  "${YELLOW_STRING}" \
  "rm --verbose failed because files do not ext" $'\n' \
  "\t${GREEN}OUTPUT_FILES${RESET}=${OUTPUT_FILES}" $'\n'
# rm --verbose --recursive "${OUTPUT_FILES[@]}";
# rm --verbose --recursive "${OUTPUT_FILES[*]}";
echo

printf '%s' \
  "${GREEN}OUTPUT_FILE_BASENAME${RESET}=" \
  "$(ls -1C "${OUTPUT_FILE_BASENAME}")" \
  $'\n'

for x in $(seq ${RANDOM_GENERATED_STRINGS_TOTAL}); do
  # shellcheck disable=SC2005,SC2086,SC2046
  echo $(shuf -r -n10 /usr/share/dict/words)
done >"${OUTPUT_FILE_BASENAME}/elements.txt"

for x in $(seq ${RANDOM_GENERATED_STRINGS_TOTAL}); do
  for y in $(seq 64); do
    printf "%d" $((RANDOM % 2))
  done
  echo
done >"${OUTPUT_FILE_BASENAME}/elements_binary.txt"

for x in $(seq ${RANDOM_GENERATED_STRINGS_TOTAL}); do
  echo "obase=16; ibase=2; $(for y in $(seq 32); do
    printf "%d" $((RANDOM % 2))
  done)" | bc
done >"${OUTPUT_FILE_BASENAME}/elements_hex.txt"

printf '%s' \
  "${GREEN}OUTPUT_FILE_BASENAME${RESET}=" \
  "$(ls -1C "${OUTPUT_FILE_BASENAME}")" \
  $'\n'

echo

# shellcheck disable=SC2016
echo -ne "${OUTPUT_FILE_BASENAME}/elements"{,_binary,_hex}.txt | xargs -d' ' -I{} bash -c 'echo "FILE=<{}>, CONTENTS=<$(cat {})>"'
