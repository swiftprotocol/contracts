MSG=$(cat <<EOF
{
  "admins": [$ADMIN],
  "denom": $CW20,
  "withdrawal_address": $ADMIN,
  "trust_contract": $TRUST,
}
EOF
)

junod tx wasm instantiate $TRUST_CODE_ID "$MSG" --label "SwiftCommerce" \
 --gas-prices 0.025ustars --gas auto --gas-adjustment 1.9 \
 --from $TESTNET_KEY --no-admin -y -b block -o json | jq .
 
