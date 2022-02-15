# Counter App

This is an example project to show how the [contract-version](https://github.com/nearcomponents/contract-version) library can be used. Please check that library for more details.

This contract has an internal counter that can be incremented, and also has a view method to show information gathered at compile-time.

## Live Testing

Download and extract the latest [release](https://github.com/nearcomponents/contract-version-example/releases), _dev-deploy_ the `example_counter.wasm` and then `near view` the `version` method.

Alternatively, you can use a live contract:
```bash
near view "dev-1644939364756-75367548411779" "version"
```

output:
```json
{
  "name": "example-counter",
  "semver": "0.0.2",
  "git_sha": "5e9df2b96ded9a80c21c0609d27c36758d2bcfca",
  "git_datetime": "2022-02-15 12:03:35 -0300",
  "git_dirty": true,
  "cargo_features": "default",
  "cargo_profile": "release",
  "rustc_semver": "1.56.1",
  "rustc_llvm": "13.0",
  "rustc_sha": "59eed8a2aac0230a8b53e89d4e99d55912ba6b35"
}
```

## Build

Build the wasm binary:

```bash
cargo build --release --target wasm32-unknown-unknown
```

## Test

After having built the wasm binary, copy them to the `res/` directory:

```bash
find target/wasm32-unknown-unknown/release \
    -maxdepth 1 \
    -name \*.wasm \
    -prune \
    -exec cp {} res \;
```

Then run the simulation tests:

```bash
cargo test
```
