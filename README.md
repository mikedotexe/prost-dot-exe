# Protobufs but less scary

Alright, so take a protobuf file, like from:

https://github.com/cosmos/cosmos-sdk/tree/main/proto/cosmos

and then strip it down so it looks super bare. See `protos/authz_for_osmosis.proto` for an example of a naked file.

Head over to `build.rs` and change obvious stuff.
Head over to `src/main.rs` and change the `include!` to your file that doesn't exist yet, which is fine.

Then:

    cargo build

will create the file in `protos-in-rust`.

Then:

    cargo run

and you'll see some output that's helpful when launching your contracts onto Osmosis mainnet using a cw3 fixed multisig.
