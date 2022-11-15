MSG=$(cat <<EOF
{
  "update_marketing": {
    "marketing": {
      "name": "$1",
      "featured_listings": [],
      "socials": []
    }
  }
}
EOF
)

junod tx wasm execute $COMMERCE "$MSG" \
  --gas-prices 0.025ujunox --gas auto --gas-adjustment 1.9 \
  --from $TESTNET_KEY -b block -y -o json | jq .
 
