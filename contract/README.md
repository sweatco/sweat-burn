# SWEAT Burn

The smart contract allows only to burn fungible tokens belonging to it.

# Quickstart

1. Make sure you have installed [rust](https://rust.org/).
2. Install the [`NEAR CLI`](https://github.com/near/near-cli#setup)

<br />

## 1. Build and Deploy the Contract
First you need to build the contract:

```bash
./build.sh
```

Then deploy it and initialize:

```bash
near dev-deploy --wasmFile res/sweat_burn.wasm --initFunction new --initArgs '{"token_account_id": "<ft_contract_id>", "manager_account_id": <manager_id>}
```

- `<ft_contract_id>` is account of fungible token on which this burning contract is going to operate.
- `<manager_id>` is an only account that is authorized to call `burn` method of this contract.

Once finished, check the `neardev/dev-account` file to find the address in which the contract was deployed:

```bash
cat ./neardev/dev-account
# e.g. dev-1659899566943-21539992274727
```

Then initialize it in FT contract if needed:

```bash
near call <ft_contract_id> storage_deposit '{"account_id": "<burn_account_id>"}' --accountId <any_account> --depositYocto 2350000000000000000000  
```

<br />

## 2. Call `burn` method

Once required, call the `burn` method with particular amount of tokens:

```bash
near call <dev-account> burn '{"amount": <amount>}' -accountId <manager_id>
```

<br />
