RED="\033[31m"
BLUE="\033[34m"
RESET="\033[0m"

log() {
  printf "%sLOG%s: %s\n" "$BLUE" "$RESET" "$@"
}

error() {
  printf "%sERROR%s: %s\n" "$RED" "$RESET" "$@"
}

