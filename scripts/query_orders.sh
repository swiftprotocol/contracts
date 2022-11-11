MSG=$(cat <<EOF
{
  "orders": {}
}
EOF
)

junod q wasm contract-state smart $COMMERCE "$MSG"