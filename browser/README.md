# efficient-zk-ecdsa in halo2 ipa

## Running browser proving

_The frontend code is located under browser_benchmark/_

### 1. Compile the prover to WASM

```
curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
sh ./scripts/build_wasm.sh
```

### Get into frontend dir

```
cd ./browser_benchmark
```

### Install dependencies

```
yarn
```

### Start dev server

```
yarn dev
```
