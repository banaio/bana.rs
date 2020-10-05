# Use same version of Golang as that on my machine
# $ lsb_release -a
# No LSB modules are available.
# Distributor ID:	Ubuntu
# Description:	Ubuntu 20.04.1 LTS
# Release:	20.04
# Codename:	focal
# $ uname -a
# Linux mbana-pc1-linux 5.4.0-45-generic #49-Ubuntu SMP Wed Aug 26 13:38:52 UTC 2020 x86_64 x86_64 x86_64 GNU/Linux
# $ go version
# go version go1.13.4 linux/amd64
FROM golang:1.13.4-alpine3.10

LABEL maintainer="Mohamed Bana <m@bana.io>"

ENV TERM xterm-256color
ENV GOOS linux
ENV GOARCH amd64
ENV CGO_ENABLED 0
ENV WORKDIR /app
ENV CRAWLER_BINARY_PATH=$WORKDIR/crawler

RUN \
  apk --no-cache update --no-cache && \
  apk --no-cache add --no-cache ca-certificates bash make git ncurses

# Add "--pretty-print", "-eufxv" to the list for additional debug output
SHELL ["/bin/bash", "--verbose", "-euf", "-o", "nounset", "-o", "errexit", "-o", "noclobber", "-o", "pipefail", "-c"]

WORKDIR $WORKDIR

# Build Cache #
# Copies go.mod go.sum to /app - was originally `COPY go.mod go.sum ./`.
COPY go.??? ./
# Now download the modules which should cache go modules to avoid re-fetching when rebuilding.
RUN go mod download
# Copy all the source files and then build them, this should result in quicker build times of the Docker image.
COPY . .
###############

# Build binary, then try to print help menu then add a sym link in `/usr/bin` to it to make life easier.
RUN \
  make build && \
  chmod +x ${CRAWLER_BINARY_PATH} && \
  ${CRAWLER_BINARY_PATH} --help && \
  echo "${CRAWLER_BINARY_PATH}: binary runs" && \
  ln -s ${CRAWLER_BINARY_PATH} /usr/bin/crawler && \
  chmod +x /usr/bin/crawler && \
  /usr/bin/crawler --help && \
  echo "/usr/bin/crawler: binary runs"

ENTRYPOINT ["/bin/bash", "-euf", "-o", "nounset", "-o", "errexit", "-o", "noclobber", "-o", "pipefail", "-c"]
CMD [ \
" \n\
RESET=`tput sgr0` \n\
RED=`tput bold; tput setaf 1` \n\
GREEN=`tput bold; tput setaf 2` \n\
printf -- '%s' ${GREEN} '---- started: ' 'time=' $(date --utc -Iseconds) ${RESET} $'\n' \n\
exit_code_crawler=0 \n\
/usr/bin/crawler || (exit_code_crawler=$? && printf -- '%s' ${RED} 'ERROR: ' 'exit_code_crawler=' ${exit_code_crawler} ${RESET} $'\n') \n\
printf -- '%s' ${GREEN} '---- ended: ' 'time=' $(date --utc -Iseconds) ', ' 'exit_code_crawler=' ${exit_code_crawler} ${RESET} $'\n' \n\
" \
]
