MSG=$(cat <<EOF
{
  "create_listing": {
    "active": true,
    "price": "$2",
    "options": [],
    "attributes": {
      "name": "$1",
      "description": "$3",
      "images": ["$4"]
    }
  }
}
EOF
)

junod tx wasm execute $COMMERCE "$MSG" \
  --gas-prices 0.025ujunox --gas auto --gas-adjustment 1.9 \
  --from $TESTNET_KEY -b block -y -o json | jq .