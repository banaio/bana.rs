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
print_env
exit 0

DOWNLOAD_URL="https://dl.google.com/go/go1.12.linux-amd64.tar.gz"

print_separator_v3
printf -- "${INDENT}%b" \
  "DOWNLOAD_URL=${DOWNLOAD_URL}" $'\n'

# printf "%b" \
#   "${RED_STRING}" \
#   "install shellcheck, shellcheck=$(command -v "${CMD}")" \
#   $'\n'
# exit 0

# echo "-----------------------------------------------"
# sudo DEBIAN_FRONTEND=noninteractive apt-get -y update
# echo "-----------------------------------------------"
# sudo DEBIAN_FRONTEND=noninteractive apt-get -y install git make
# echo "-----------------------------------------------"
# cd /tmp
# wget 'https://dl.google.com/go/go1.12.linux-amd64.tar.gz'
# echo "-----------------------------------------------"
# sudo tar -C /usr/local -xzf go1.12.linux-amd64.tar.gz
# echo "-----------------------------------------------"
# rm -fr go1.12.linux-amd64.tar.gz
# echo "-----------------------------------------------"
# echo 'export PATH="${PATH}:/usr/local/go/bin"' >>"${HOME}/.profile"
# source "${HOME}/.profile"
# echo "-----------------------------------------------"
# go version
# echo "-----------------------------------------------"
