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

COMMANDS=(
  "shellcheck"
  "lld"
  "lld-10"
  "yamllint"
)
INSTALL_COMMAND=
DEBIAN_FRONTEND=
PKG_MANAGER_INSTALL_COMMAND=

if [[ "${UNAME_S}" = "Linux" ]]; then
  INSTALL_COMMAND="sudo apt-get -qq install -y --yes --assume-yes -qq"
  # INSTALL_COMMAND="sudo apt-get install -y --yes --assume-yes -qq"
  # sudo apt-get install -y --yes --assume-yes -qq openssl-devel
  # sudo apt-get -q install -y --yes --assume-yes openssl-devel
  # sudo apt-get -qq install -y --yes --assume-yes openssl-devel
  # sudo apt-get install -y --yes --assume-yes openssl-devel 1>/dev/null
  export DEBIAN_FRONTEND="noninteractive"
elif [[ "${UNAME_S}" = "Darwin" ]]; then
  INSTALL_COMMAND="brew install"
  unset DEBIAN_FRONTEND
else
  printf "%b" \
    "${RED_STRING}" \
    "INSTALL_COMMAND - unsupported OS, UNAME_S=${UNAME_S}" \
    $'\n'
  exit 1
fi
PKG_MANAGER_INSTALL_COMMAND="${INSTALL_COMMAND}"

print_separator_v3
printf -- "${INDENT}%b" \
  "INSTALL_COMMAND=${INSTALL_COMMAND}" $'\n' \
  "PKG_MANAGER_INSTALL_COMMAND=${PKG_MANAGER_INSTALL_COMMAND}" $'\n' \
  "DEBIAN_FRONTEND=${DEBIAN_FRONTEND}" $'\n' \
  "COMMANDS=${COMMANDS[*]}" $'\n'

for x in "${!COMMANDS[@]}"; do
  COMMAND="${COMMANDS[$x]}"
  echo "COMMAND=${COMMAND}"

  # COMMAND="shellcheck"
  if [[ ! -x "$(command -v "${COMMAND}" 2>/dev/null)" ]]; then
    if [[ "${UNAME_S}" = "Linux" ]]; then
      ${INSTALL_COMMAND} "${COMMAND}" 2>&1 || printf "%b" "${ERROR_STRING}" "failed to install - COMMAND=${COMMAND}, exit_code=$?" $'\n'
    elif [[ "${UNAME_S}" = "Darwin" ]]; then
      ${INSTALL_COMMAND} "${COMMAND}"
    else
      printf "%b" \
        "${RED_STRING}" \
        "unsupported OS - UNAME_S=${UNAME_S}, COMMAND=${COMMAND}" \
        $'\n'
      exit 1
    fi
  else
    # COMMAND_VERSION="$(1>/dev/null 2>&1 ${COMMAND} --version | tr -d '\n')"
    COMMAND_VERSION=$("${COMMAND}" 2>&1 --version | tr -d '\n' || true)
    printf "%b" \
      "${YELLOW_STRING}" \
      "COMMANDS - command already exists, COMMAND=${COMMAND}, COMMAND_VERSION=${COMMAND_VERSION}" \
      $'\n'
  fi
done

LINKERS=$(find /usr/bin -iname '*lld*' -o -iname '*ld-10*')
LINKERS_TOTAL=$(echo "${LINKERS}" | wc -l)

printf -- "${INDENT}%b" \
  "LINKERS=${LINKERS//$'\n'/,}" $'\n' \
  "LINKERS_TOTAL=${LINKERS_TOTAL}" $'\n'

COMMANDS_EXTRA=(
  "parallel"
  "libssl-dev"
  "libssh2-devel"
  "openssl-devel"
  "libgit2-devel"
  "parallel"
  "libssl-dev"
  "libssh-dev"
  "libgit2-dev"
  "colormake"
)
printf -- "${INDENT}%b" \
  "${GREEN}COMMANDS_EXTRA${RESET}=${COMMANDS_EXTRA[*]}" $'\n'

for x in "${!COMMANDS_EXTRA[@]}"; do
  COMMAND_EXTRA="${COMMANDS_EXTRA[$x]}"
  # echo "COMMAND_EXTRA=${COMMAND_EXTRA}"
  printf -- "${INDENT}%b" \
    "${GREEN}COMMAND_EXTRA${RESET}=${COMMAND_EXTRA}" $'\n'

  if [[ "${UNAME_S}" = "Linux" ]]; then
    # INSTALL_LOG=$(
    #   set -xv
    #   exit_code=0
    #   LOG=$(${INSTALL_COMMAND} "${COMMAND_EXTRA}" 2>&1) || exit_code=$?
    #   if [[ ${exit_code} -ne 0 ]]; then
    #     # echo "${INSTALL_COMMAND} ${COMMAND_EXTRA}, exit_code=${exit_code}, LOG=${LOG}"
    #     echo ${LOG}
    #     exit ${exit_code}
    #   fi
    #   exit ${exit_code}
    # )
    EXIT_CODE=0
    INSTALL_LOG=$(
      ${INSTALL_COMMAND} "${COMMAND_EXTRA}" 2>&1
    ) || EXIT_CODE=$?
    if [[ ${EXIT_CODE} -ne 0 ]]; then
      printf "%b" "${ERROR_STRING}" "failed to install - COMMAND_EXTRA=${COMMAND_EXTRA}, EXIT_CODE=${EXIT_CODE}, INSTALL_LOG=${RED}${INSTALL_LOG}${RESET}" $'\n'
    fi
    # ${INSTALL_COMMAND} "${COMMAND_EXTRA}" || printf "%b" "${ERROR_STRING}" "failed to install - COMMAND_EXTRA=${COMMAND_EXTRA}, exit_code=$?" $'\n'
  elif [[ "${UNAME_S}" = "Darwin" ]]; then
    printf "%b" \
      "${RED_STRING}" \
      "unsupported OS - UNAME_S=${UNAME_S}, COMMAND_EXTRA=${COMMAND_EXTRA}" \
      $'\n'
    exit 1
  else
    printf "%b" \
      "${RED_STRING}" \
      "unsupported OS - UNAME_S=${UNAME_S}, COMMAND_EXTRA=${COMMAND_EXTRA}" \
      $'\n'
    exit 1
  fi
done

# # Install additional packages, if Linux. Not sure about Darwin yet.
# if [[ "${UNAME_S}" = "Linux" ]]; then
#   ${INSTALL_COMMAND} "${COMMANDS_EXTRA[@]}"
# elif [[ "${UNAME_S}" = "Darwin" ]]; then
#   # ${INSTALL_COMMAND} "${COMMANDS_EXTRA[@]}"
#   printf "%b" \
#     "${RED_STRING}" \
#     "COMMANDS_EXTRA - Unsupported OS, UNAME_S=${UNAME_S}" \
#     $'\n'
#   exit 1
# else
#   printf "%b" \
#     "${RED_STRING}" \
#     "COMMANDS_EXTRA - Unsupported OS, UNAME_S=${UNAME_S}" \
#     $'\n'
#   exit 1
# fi

print_separator
printf "%b" \
  "${GREEN_STRING}" "install complete" $'\n'

exit 0
