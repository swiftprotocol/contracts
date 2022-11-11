MSG=$(cat <<EOF
{
  "order": {
    "id": $1
  }
}
EOF
)

junod q wasm contract-state smart $COMMERCE "$MSG"