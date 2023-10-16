if [[ $(uname -a) == *"Microsoft"* ]]; then
  echo "You are on WSL/Windows."
else
  echo "You are on Linux."
fi