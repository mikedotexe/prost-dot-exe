# Protobufs but less scary

Alright, so take a protobuf file, like from:

https://github.com/cosmos/cosmos-sdk/tree/main/proto/cosmos

and then strip it down so it looks super bare. See `protos/authz_for_osmosis.proto` for an example of a naked file.

Head over to `build.rs` and change obvious stuff.

Then:

    cargo build

This will create your file in the `protos-in-rust` directory.

Now head over to `src/main.rs` and change the `include!` to your file that doesn't exist yet, which is fine.

Then:

    cargo run

and you'll see some output that's helpful when launching your contracts onto Osmosis mainnet using a cw3 fixed multisig.

## Troubleshooting

There's kind of a chicken and egg problem, where to build the file, cargo will also check that `main.rs` will compile. And it won't compile if you don't have or remove the `authz_for_osmosis.rs` file.
