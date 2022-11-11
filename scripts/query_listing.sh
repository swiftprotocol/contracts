MSG=$(cat <<EOF
{
  "listing": {
    "id": $1
  }
}
EOF
)

junod q wasm contract-state smart $COMMERCE "$MSG"