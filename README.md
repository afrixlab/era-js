# era-js

A WebAssembly-powered cryptographic toolkit for the ERA project, providing secure wallet shard management and Reed-Solomon erasure coding for robust key recovery.

## Features

- **WebAssembly (WASM) bindings** for high-performance cryptographic operations in JavaScript.
- **Reed-Solomon erasure coding** for secure wallet shard reconstruction and fault tolerance.
- **Serde-based deserialization** for seamless integration with JavaScript objects.
- **Dual-licensed** under MIT and Apache-2.0 for maximum compatibility.

## Installation

Install via npm (after building with `wasm-pack`):

```sh
npm install era-js
```

Or use directly in your JavaScript/TypeScript project by importing the generated WASM and JS files.

## Usage

### Importing

```js
import init, { BaseWallet } from 'era-js';
// or, if using the generated JS directly:
import { BaseWallet } from './era_js.js';
```

### Example: Reconstructing Wallet Shards

```js
// Example wallet shards (Uint8Array or array of numbers)
const walletData = {
  project_shard: [/* ... */],
  system_shard: [/* ... */],
  recovery_shard: [/* ... */]
};

const wallet = new BaseWallet(walletData);
const reconstructed = wallet.reconstruct_shards();
// `reconstructed` is a Uint8Array containing the combined data shards
```

## API

### `BaseWallet`

#### Constructor

```typescript
new BaseWallet(value: object): BaseWallet
```
- `value`: An object with optional `project_shard`, `system_shard`, and `recovery_shard` fields (each a `Uint8Array` or array of bytes).

#### Methods

- `reconstruct_shards(): Uint8Array`
  - Reconstructs the original data from the provided shards using Reed-Solomon coding.
  - Throws if reconstruction or verification fails.

## How It Works

1. **Shard Storage:** The wallet generates up to three shards: `project_shard`, `system_shard`, and `recovery_shard`.
2. **Reconstruction:** Using Reed-Solomon coding (2 data shards, 3 parity shards), the wallet can reconstruct the original key material even if some shards are missing.
3. **Verification:** The reconstructed shards are verified for integrity before combining.

## Building from Source

Ensure you have [Rust](https://www.rust-lang.org/tools/install) and [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/) installed.

```sh
wasm-pack build
```

This will generate the WASM and JS bindings in the `pkg/` directory.

## Testing

Run Rust tests:

```sh
cargo test
```

Or test the WASM package in headless browsers:

```sh
wasm-pack test --headless --firefox
```

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE_APACHE](LICENSE_APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE_MIT](LICENSE_MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.

---

For more details, see the [pkg/README.md](pkg/README.md) and [src/era/wallet.rs](src/era/wallet.rs) files.