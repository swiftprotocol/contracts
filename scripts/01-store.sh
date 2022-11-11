find . -name "*.wasm" -type f|xargs rm -f

curl -s https://api.github.com/repos/swiftprotocol/contracts/releases/latest \
| grep ".*wasm" \
| cut -d : -f 2,3 \
| tr -d \" \
| wget -qi -

junod config node $NODE
junod config chain-id $CHAIN_ID
junod config output json

echo "- commerce\n";
junod tx wasm store commerce.wasm --from $TESTNET_KEY \
    --gas-prices 0.025ustars --gas-adjustment 1.7 \
    --gas auto -y -b block -o json | jq '.logs' | grep -A 1 code_id

echo "- trust\n";
junod tx wasm store trust.wasm --from $TESTNET_KEY \
    --gas-prices 0.025ustars --gas-adjustment 1.7 \
    --gas auto -y -b block -o json | jq '.logs' | grep -A 1 code_id
