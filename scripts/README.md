# Swift Protocol Deployment Scripts

## Setup

### Install and update `junod`

If not already done, install the `junod` binary:

```bash
git clone https://github.com/cosmos-contracts/juno
cd juno
git checkout v11.0.0
make install
```

This requires an install of `golang` on your machine. Verify that it exists by running `go version`. Make sure this is at least `1.18.0`.

If you already have `junod` installed, verify that it is at least version `v11.0.1` by running:

```bash
junod version
```

To update `junod` to 11 or higher, run the same steps as outlined in the section for users with no binary installed.

Once done, make sure to run `source ~/.zshrc` or the rc file for whichever shell you are using. If you are running macOS, this will most likely be `.zshrc`, for Linux most likely `.bashrc`. If you are using Windows, nobody loves you.

### Set environment variables

```bash
cp .env.example .env
```

Set your environment variables in `.env` and run `source .env` before running any script. Re-run this command every time you make a change to `.env`.

### CW20 token

Create a token-based DAO on [DAODAO](https://testnet.daodao.zone).

Get the CW20 token address and set it to `CW20` in the environment variable file, then get the CW20 staking contract address for your token and set it to `CW20_STAKE` in the env file.

Run `source .env` to refresh the environment variable file.

### Store code on-chain

```bash
./01-store.sh
```

Set the code IDs you get from this script in `.env`.

### Instantiate trust contract

```bash
./02-init_trust.sh
```

Get the contract address from the result JSON and add it to `.env`.

### Instantiate commerce contract

```bash
./03-init_commerce.sh
```

Once again, get the contract address from the result JSON and add it to `.env`.

## Functionality

### Create a product

```bash
./exec_create_listing.sh [name] [price] [description] [image]
```

### Create an order for a product

```bash
./exec_create_order.sh [product_id]
```

The buyer account will need the exact amount of the product's price to be available in CW20 tokens with the CW20 address defined in `.env`.

### Update an order

```bash
./exec_update_order.sh [order_id]
```

For this command, you will have to enter the details you want to update in the `exec_update_order.sh` file before running it.

### Complete an order

```bash
./exec_complete_order.sh [order_id]
```

### Leave a review to another user

```bash
./exec_review.sh [address] [review]
```

`review` should be either "ThumbsUp" or "ThumbsDown". Any other string will produce an error.

## Queries

### Query an address' trust info

```bash
./query_trust.sh [address]
```

### Query all of a site's listings

```bash
./query_listings.sh
```

### Query product listing info

```bash
./query_listing [id]
```

### Query all of a site's active orders

```bash
./query_orders.sh
```

### Query order details

```bash
./query_order.sh [id]
```

## Deployment

### Generate schemas

```bash
./schema.sh
```

### Optimize contacts

```bash
./optimize.sh
```

### Publish crates

```bash
./publish.sh
```
