RED="\033[31m"
BLUE="\033[34m"
RESET="\033[0m"

log() {
  echo -e "${BLUE}LOG${RESET}: $*"
  # printf "%sLOG%s: %s\n" "$BLUE" "$RESET" "$@"
}

error() {
  echo -e "${RED}ERROR${RESET}: $*"
  # printf "%sERROR%s: %s\n" "$RED" "$RESET" "$@"
}

