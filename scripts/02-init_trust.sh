MSG=$(cat <<EOF
{
  "maintainer": "$ADMIN",
  "staking_contract": "$CW20_STAKE",
  "commerce_code_id": $COMMERCE_CODE_ID,
  "review_interval": 86400,
  "max_staked_tokens": "5000000000",
  "max_staked_days": 240,
  "trust_score_params": {
    "base_score": 500,
    "rating_multiplier": 25,
    "stake_amount_denominator": 2500,
    "min_stake_days": 14,
    "rating_floor_denominator": 10,
    "denom_multiplier": "1000000"
  },
  "max_rating": 50
}
EOF
)

junod tx wasm instantiate $TRUST_CODE_ID "$MSG" --label "SwiftTrust" \
 --gas-prices 0.025ujunox --gas auto --gas-adjustment 1.9 \
 --from $TESTNET_KEY --no-admin -y -b block -o json | jq .
 
