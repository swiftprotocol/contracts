MSG=$(cat <<EOF
{
  "trust_info": {
    "address": "$1"
  }
}
EOF
)

junod q wasm contract-state smart $TRUST "$MSG"