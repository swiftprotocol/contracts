MSG=$(cat <<EOF
{
  "listings": {}
}
EOF
)

junod q wasm contract-state smart $COMMERCE "$MSG"