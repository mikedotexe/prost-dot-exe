// Why extern crate? Who fucking knows and it's amazing.
extern crate base64;
extern crate chrono;
extern crate core;
extern crate prost;
extern crate prost_types;
extern crate serde_json;

include!(concat!(
env!("CARGO_MANIFEST_DIR"),
"/protos-in-rust/authz_for_osmosis.rs"
));
// base64 crate feels weird, but this is normal.
use base64::engine::general_purpose::STANDARD as BASE64;
use base64::engine::Engine as _;

use chrono::{Duration, Utc};
// yep, you do need this
use prost::Message;
use prost_types::{Any, Timestamp};
use serde_json::json;

// PLEASE SET THESE VALUES
pub const GRANTER: &str = "osmo15wna5dwylkuzvljsudyn6zfsd4zl0rkg5ge888mzk4vtnjpp0z5q4e9w58";
// Mike's EOA associated with mikedotexe.osmo on ICNS
pub const GRANTEE: &str = "osmo1yhqft6d2msmzpugdjtawsgdlwvgq3samajy9jq";
pub const NUMBER_OF_DAYS_GRANTED: u8 = 1;

fn main() {
    let now = Utc::now();
    let two_weeks_from_now = (now + Duration::days(NUMBER_OF_DAYS_GRANTED as i64)).timestamp();

    let generic_authorization = GenericAuthorization {
        msg: "/cosmwasm.wasm.v1.MsgStoreCode".to_string(),
    };

    let mut generic_authorization_bytes = Vec::new();
    generic_authorization
        .encode(&mut generic_authorization_bytes)
        .expect("Failed encoding GenericAuthorization");

    let authorization = Any {
        type_url: "/cosmos.authz.v1beta1.GenericAuthorization".to_string(),
        value: generic_authorization_bytes,
    };

    let grant = Grant {
        authorization: Some(authorization),
        expiration: Some(Timestamp {
            seconds: two_weeks_from_now,
            nanos: 0,
        }),
    };

    let msg_grant = MsgGrant {
        granter: GRANTER.to_string(),
        grantee: GRANTEE.to_string(),
        grant: Some(grant),
    };

    let mut msg_grant_bytes = Vec::new();
    msg_grant
        .encode(&mut msg_grant_bytes)
        .expect("Failed encoding MsgGrant");

    let base64_string = BASE64.encode(&msg_grant_bytes);

    // This is the message you'll send to the cw3 fixed multisig
    let msg_json = json!({
      "propose": {
        "title": format!("Allow {} to Upload Code for {} day(s)", GRANTEE, NUMBER_OF_DAYS_GRANTED),
        "description": format!("Gives authorization to {} to upload contracts on behalf of this multisig for {} day(s).", GRANTEE, NUMBER_OF_DAYS_GRANTED),
        "msgs": [
          {
            "stargate": {
              "type_url": "/cosmos.authz.v1beta1.MsgGrant",
              "value": base64_string
            }
          }
        ],
        "latest": null
      }
    });

    println!("Base64 Stargate value:\n\n{}\n", base64_string);
    println!("------ ------ ------");
    // give some nice padding, yes, there we go
    println!("cw3-fixed-multisig Propose Message. See:\nhttps://docs.rs/cw3-fixed-multisig/latest/cw3_fixed_multisig/msg/enum.ExecuteMsg.html#variant.Propose\n\n{}\n", serde_json::to_string_pretty(&msg_json).expect("Couldn't make our JSON purty"));

    println!("------ ------ ------");
    println!("In one line:\n");
    println!("{}\n", msg_json);

    // Can see this happen with Kado if you wish
    // reproduce_kado();
}

// Actual example to compare:
// https://www.mintscan.io/osmosis/txs/3E355815C6533B1575FC6A274CE1FCD14F996F4B89BB25F1EB709D73ADD5C82C?height=9428169
pub const KADO_GRANTER: &str = "osmo19vxp8vq8qm368dr026qxh8v82satwaf79y235lfv6wmgpwxx8dtskedaku";
pub const KADO_GRANTEE: &str = "osmo18h47lm65q0r02gcuxe6vslk8u5ftgrl9wrtea6";

#[allow(dead_code)]
fn reproduce_kado() {
    let gen_auth_store_code = GenericAuthorization {
        msg: "/cosmwasm.wasm.v1.MsgStoreCode".to_string(),
    };

    let mut generic_authorization_bytes = Vec::new();
    gen_auth_store_code
        .encode(&mut generic_authorization_bytes)
        .expect("Couldn't encode with your message");

    let authorization = Any {
        type_url: "/cosmos.authz.v1beta1.GenericAuthorization".to_string(),
        value: generic_authorization_bytes,
    };

    // These taken from the deserialized "value" from Mintscan
    let grant = Grant {
        authorization: Some(authorization),
        expiration: Some(Timestamp {
            seconds: 1683559426,
            nanos: 416000000,
        }),
    };

    let msg_grant = MsgGrant {
        granter: KADO_GRANTER.to_string(),
        grantee: KADO_GRANTEE.to_string(),
        grant: Some(grant),
    };

    let mut msg_grant_bytes = Vec::new();
    msg_grant
        .encode(&mut msg_grant_bytes)
        .expect("Message grant encoding failed");

    let base64_string = BASE64.encode(&msg_grant_bytes);

    println!("value in base64: {}", base64_string);

    // Again, taken from: https://www.mintscan.io/osmosis/txs/3E355815C6533B1575FC6A274CE1FCD14F996F4B89BB25F1EB709D73ADD5C82C?height=9428169
    let base64_payload = "Cj9vc21vMTl2eHA4dnE4cW0zNjhkcjAyNnF4aDh2ODJzYXR3YWY3OXkyMzVsZnY2d21ncHd4eDhkdHNrZWRha3USK29zbW8xOGg0N2xtNjVxMHIwMmdjdXhlNnZzbGs4dTVmdGdybDl3cnRlYTYaXgpOCiovY29zbW9zLmF1dGh6LnYxYmV0YTEuR2VuZXJpY0F1dGhvcml6YXRpb24SIAoeL2Nvc213YXNtLndhc20udjEuTXNnU3RvcmVDb2RlEgwIgqjkogYQgNCuxgE=";

    if base64_string == base64_payload {
        println!("These are the same, brah.");
    }

    // Decode the base64 payload
    let decoded_bytes = BASE64
        .decode(base64_payload)
        .expect("Failed to decode base64 payload");

    // Parse the decoded bytes into a MsgGrant message
    let kado_msg_grant =
        MsgGrant::decode(&*decoded_bytes).expect("Failed to parse MsgGrant message");

    // Print the parsed message
    println!("kado_msg_grant {:?}", kado_msg_grant);
}
