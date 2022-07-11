# near-counter-example

## Git clone

```bash
git clone https://github.com/DSRV-DevGuild/near-counter-example.git

```

## Set up environments

### Install Rust

```bash
# install rustup
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
# configure your current shell
source $HOME/.cargo/env
# add wasm target to your toolchain
rustup target add wasm32-unknown-unknown
```

### Install near-cli

```bash
npm install -g near-cli
```

### Login

If you do not have a NEAR account, please create one with [NEAR Wallet](https://wallet.testnet.near.org).

```
near login
```

---

## Compile

```bash
# AssemblyScript
cd AssemblyScript && yarn install
yarn asb

# Rust
cd Rust
cargo build --target wasm32-unknown-unknown --release
```

## Deploy

```bash
# near deploy
## AssemblyScript
near deploy --wasmFile build/release/counter_contract.wasm --accountId YOUR_ACCOUNT_HERE
## Rust
near deploy --wasmFile target/wasm32-unknown-unknown/release/counter_contract.wasm --accountId YOUR_ACCOUNT_HERE

# near dev-deploy (testnet only)
## AssemblyScript
near dev-deploy build/release/counter_contract.wasm
## Rust
near dev-deploy target/wasm32-unknown-unknown/release/counter_contract.wasm
```
