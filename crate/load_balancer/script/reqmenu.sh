#!/bin/sh

# Change to the controller folder
script_dir="$(dirname "$(realpath "$0")")/"
cd "${script_dir}../src/controller/"

# Temporary files
temp_grep_output=$(mktemp)
temp_menu=$(mktemp)
temp_result=$(mktemp)

log() {
  type=$1
  text=$2
  echo "$type: $text"
}

index=0
while read -r file; do
  grep -oP '#\[\w+\("\K[^"]+' "$file" > "$temp_grep_output"
  while read -r first_arg; do
    req_type=$(grep -B1 "$first_arg" "$file" | grep -oP '#\[\K\w+')
    case "$req_type" in
      get|post|put|delete|head|options|patch|connect)
        fn_name=$(awk "/#\[$req_type\(\"$( echo $first_arg | sed 's/\//\\\//g')\"/ {getline; print}" "$file" | grep -oP 'fn \K\w+')
        line_number=$(grep -n "#\[$req_type(\"$first_arg\"" "$file" | cut -f1 -d:)
        log debug "$line_number"
        echo "$index \"${file%.rs} -> $req_type -> $fn_name -> $first_arg\"" >> "$temp_menu"
        echo "$index $file:$line_number" >> "$temp_result"
        index=$((index + 1))
        ;;
    esac
  done < "$temp_grep_output"
done < <(find . -name '*.rs')

log debug "$(cat $temp_menu)"

index=$(mktemp)

while true; do
  # Dialog menu
  dialog --clear --title "Select Function" --menu "Choose one:" 20 60 15 --file "$temp_menu" 2> $index
  log debug $(cat $index)

  # Exit on "Cancel"
  if [ $? -eq 1 ]; then
    break
  fi

  # Output selected option
  selection="$(cat "$index")"
  selected_line="$(awk -v idx="$selection" '$1==idx { print $2 }' "$temp_result")"

  # Show selected data and return to menu
  dialog --clear --title "You selected:" --msgbox "File and Line: $selected_line\nPress OK to return to menu." 10 40
done

# Cleanup
rm -f "$temp_file" "$temp_menu" "$temp_result"
