

export const secretDevInfo = {
    chainId: "secretdev-1",
    chainName: "LocalSecret",
    rpc: "http://localhost:26657",
    rest: "http://localhost:1317",
    bip44: {
      coinType: 529,
    },
    bech32Config: {
      bech32PrefixAccAddr: "secret",
      bech32PrefixAccPub: "secretpub",
      bech32PrefixValAddr: "secretvaloper",
      bech32PrefixValPub: "secretvaloperpub",
      bech32PrefixConsAddr: "secretvalcons",
      bech32PrefixConsPub: "secretvalconspub",
    },
    currencies: [
      {
        coinDenom: "SCRT",
        coinMinimalDenom: "uscrt",
        coinDecimals: 6,
        coinGeckoId: "secret",
      },
    ],
    feeCurrencies: [
      {
        coinDenom: "SCRT",
        coinMinimalDenom: "uscrt",
        coinDecimals: 6,
        coinGeckoId: "secret",
      },
    ],
    stakeCurrency: {
      coinDenom: "SCRT",
      coinMinimalDenom: "uscrt",
      coinDecimals: 6,
      coinGeckoId: "secret",
    },
    coinType: 529,
    gasPriceStep: {
      low: 0.1,
      average: 0.25,
      high: 1,
    },
    features: ["secretwasm", "stargate", "ibc-transfer", "ibc-go"],
  
};