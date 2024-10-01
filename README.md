# CosmWasm ISei Silohub Query Contract

This is a CosmWasm smart contract that allows you to query the `Silohub` contract on native sei. Silohub contract allows a user to query the contract to get the exchange rate of `ISei` to `Sei`.

## Silohub Address

The contract address of the `Silohub` contract is `sei1e3gttzq5e5k49f9f5gzvrl0rltlav65xu6p9xc0aj7e84lantdjqp7cncc`.

## Deployed Contract Address

The contract address of the deployed `ISei Silohub Query` contract is `sei1a7xygdcf8f0ztdlglh9kme7nm3l4x6avmmjxe3ss57wxc4j6vuhqqa453z`.

## Contract Workflow

The contract workflow is as follows:

1. The contract queries the `Silohub` contract to get the exchange rates of `ISei` to `Sei`.
2. The `Silohub` contract returns the exchange rates to the contract in the following format:

```json
{
  "data": {
    "exchange_rates": [
      [
        1727762390,
        "1.033207463698531844"
      ]
    ],
    "apr": null
  }
}
```

3. The response is parsed by the contract and processed to return the first item in the `exchange_rates` array which is the latest exchange rate of `ISei` to `Sei`.
4. The contract returns the processed exchange rate to the caller.