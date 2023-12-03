input_string=$1
key=$2
hash_length=$3  # Desired length of the remaining hash part

full_hash=$(echo -n "$input_string" | openssl dgst -sha256 -hmac "$key")
echo "${full_hash:17:$hash_length}"