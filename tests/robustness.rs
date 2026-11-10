//! NFR-5 / AC-9: no input can crash the service or produce a 5xx.
mod common;
use common::*;
use proptest::prelude::*;
use serde_json::json;

fn op() -> impl Strategy<Value = String> {
    prop_oneof![
        Just("add"),
        Just("subtract"),
        Just("multiply"),
        Just("divide"),
        Just("modulo"),
        Just("power"),
        Just("percent"),
        Just("sqrt"),
    ]
    .prop_map(String::from)
}

/// Mostly valid-looking decimals (including huge and tiny ones), plus arbitrary strings.
fn operand() -> impl Strategy<Value = String> {
    prop_oneof![
        3 => "-?[0-9]{1,30}(\\.[0-9]{1,30})?",
        1 => any::<String>(),
    ]
}

fn runtime() -> tokio::runtime::Runtime {
    tokio::runtime::Builder::new_current_thread()
        .build()
        .unwrap()
}

fn assert_well_formed(status: StatusCode, body: &serde_json::Value) {
    assert!(!status.is_server_error(), "5xx: {status} {body}");
    if status.is_success() {
        assert!(body["result"].is_string(), "{body}");
    } else {
        assert!(body["error"]["code"].is_string(), "{status} {body}");
    }
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(512))]

    #[test]
    fn nfr5_any_operands_never_5xx(op in op(), a in operand(), b in proptest::option::of(operand())) {
        let mut req = json!({ "op": op, "a": a });
        if let Some(b) = b {
            req["b"] = json!(b);
        }
        let (status, body) = runtime().block_on(calc(req));
        assert_well_formed(status, &body);
    }

    #[test]
    fn nfr5_arbitrary_bodies_never_5xx(raw in any::<Vec<u8>>()) {
        let (status, body) = runtime().block_on(post_raw("/v1/calculate", raw));
        assert_well_formed(status, &body);
    }
}
