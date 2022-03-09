## Install Rust.

Open up a terminal and punch in the following installation command from the official rust servers.


```curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh```

Once installed, we’ll need to install the WebAssembly toolchain. This will enable us to compile Rust into a WebAssembly file that can be run on the blockchain.


```rustup target add wasm32-unknown-unknown```

<hr>

### Compile :
```sh scripts/build.sh```
### deploy : 
```near dev-deploy --wasmFile ../result/result.wasm```

### Testing : 
```cargo test -- --nocapture```

### Contract Interaction

>   ```near call $CONTRACT set_owner '{"token_id": "firstNFT", "account_id" : "<account_id>.testnet"}' --accountId "<account_id>.testnet"```
<hr>

> ```near view $CONTRACT get_owner '{"token_id": "firstNFT"}'```
