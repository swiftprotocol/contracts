# We need to get the price of the order first

# In a situation where the listing has options selected in the order,
# we'd also have to query the options' prices and do some math.
# For the sake of simplicity, we assume there are no options.
# This should be the case if the listing was created using exec_create_listing.sh

MSG=$(cat <<EOF
{
  "listing": {
    "id": $1
  }
}
EOF
)

junod q wasm contract-state smart $COMMERCE "$MSG" | jq '.price.amount'