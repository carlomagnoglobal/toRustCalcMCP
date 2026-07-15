//! Integration tests for the MCP JSON-RPC layer (`mcp::handle_message`).

use serde_json::{json, Value as J};
use torustcalcmcp::eval::Interp;
use torustcalcmcp::mcp;

fn call(it: &mut Interp, msg: J) -> Option<J> {
    mcp::handle_message(it, &msg)
}

fn tool_call(it: &mut Interp, id: i64, name: &str, args: J) -> J {
    let resp = call(
        it,
        json!({
            "jsonrpc": "2.0",
            "id": id,
            "method": "tools/call",
            "params": { "name": name, "arguments": args }
        }),
    )
    .expect("tools/call must produce a response");
    resp["result"].clone()
}

#[test]
fn initialize_returns_server_info() {
    let mut it = Interp::new();
    let resp = call(
        &mut it,
        json!({"jsonrpc":"2.0","id":1,"method":"initialize","params":{}}),
    )
    .unwrap();
    let result = &resp["result"];
    assert_eq!(result["protocolVersion"], mcp::PROTOCOL_VERSION);
    assert_eq!(result["serverInfo"]["name"], "toRustCalcMCP");
    assert!(result["capabilities"]["tools"].is_object());
}

#[test]
fn notification_gets_no_response() {
    let mut it = Interp::new();
    let resp = call(
        &mut it,
        json!({"jsonrpc":"2.0","method":"notifications/initialized"}),
    );
    assert!(resp.is_none());
}

#[test]
fn unknown_method_returns_32601() {
    let mut it = Interp::new();
    let resp = call(
        &mut it,
        json!({"jsonrpc":"2.0","id":9,"method":"no/such/method"}),
    )
    .unwrap();
    assert_eq!(resp["error"]["code"], -32601);
}

#[test]
fn tools_list_has_four_tools_with_output_schemas() {
    let mut it = Interp::new();
    let resp = call(&mut it, json!({"jsonrpc":"2.0","id":2,"method":"tools/list"})).unwrap();
    let tools = resp["result"]["tools"].as_array().unwrap();
    assert_eq!(tools.len(), 4);
    for t in tools {
        assert!(
            t["outputSchema"].is_object(),
            "tool {} missing outputSchema",
            t["name"]
        );
    }
    // calc_config must declare ibase/obase so schema-validating clients can send them
    let config = tools
        .iter()
        .find(|t| t["name"] == "calc_config")
        .expect("calc_config tool");
    let props = &config["inputSchema"]["properties"];
    assert!(props["ibase"].is_object());
    assert!(props["obase"].is_object());
}

#[test]
fn calc_eval_exact_big_power_with_structured_content() {
    let mut it = Interp::new();
    let result = tool_call(&mut it, 3, "calc_eval", json!({"expression":"2^256"}));
    assert_eq!(result["isError"], false);
    let text = result["content"][0]["text"].as_str().unwrap();
    assert_eq!(
        text,
        "115792089237316195423570985008687907853269984665640564039457584007913129639936"
    );
    let sc = &result["structuredContent"];
    assert_eq!(sc["expression"], "2^256");
    assert_eq!(sc["result"], text);
    assert_eq!(sc["mode"], "real");
}

#[test]
fn calc_eval_error_sets_is_error() {
    let mut it = Interp::new();
    let result = tool_call(&mut it, 4, "calc_eval", json!({"expression":"1/0"}));
    assert_eq!(result["isError"], true);
    let text = result["content"][0]["text"].as_str().unwrap();
    assert!(text.starts_with("error:"));
}

#[test]
fn calc_config_set_persists_bases_and_mode() {
    let mut it = Interp::new();
    let set = tool_call(
        &mut it,
        5,
        "calc_config",
        json!({"action":"set","mode":"frac","ibase":16,"obase":16}),
    );
    assert_eq!(set["isError"], false);
    let get = tool_call(&mut it, 6, "calc_config", json!({"action":"get"}));
    let sc = &get["structuredContent"];
    assert_eq!(sc["mode"], "frac");
    assert_eq!(sc["ibase"], 16);
    assert_eq!(sc["obase"], 16);
}

#[test]
fn calc_config_obase_affects_eval_rendering() {
    let mut it = Interp::new();
    tool_call(&mut it, 7, "calc_config", json!({"action":"set","obase":16}));
    let result = tool_call(&mut it, 8, "calc_eval", json!({"expression":"255"}));
    assert_eq!(result["content"][0]["text"], "ff");
}

#[test]
fn calc_functions_filter_returns_structured_list() {
    let mut it = Interp::new();
    let result = tool_call(&mut it, 9, "calc_functions", json!({"filter":"sha1"}));
    assert_eq!(result["isError"], false);
    let sc = &result["structuredContent"];
    let funcs = sc["functions"].as_array().unwrap();
    assert!(!funcs.is_empty());
    assert_eq!(sc["count"], funcs.len());
    for f in funcs {
        assert!(f["name"].is_string());
        assert!(f["signature"].is_string());
        assert!(f["description"].is_string());
    }
    assert_eq!(funcs[0]["name"], "sha1");
}

#[test]
fn calc_session_state_and_reset() {
    let mut it = Interp::new();
    tool_call(&mut it, 10, "calc_eval", json!({"expression":"x = 42"}));
    let state = tool_call(&mut it, 11, "calc_session", json!({"action":"state"}));
    assert_eq!(state["structuredContent"]["variables"], 1);
    let reset = tool_call(&mut it, 12, "calc_session", json!({"action":"reset"}));
    assert_eq!(reset["structuredContent"]["action"], "reset");
    let state2 = tool_call(&mut it, 13, "calc_session", json!({"action":"state"}));
    assert_eq!(state2["structuredContent"]["variables"], 0);
}

#[test]
fn calc_eval_overrides_do_not_leak_into_session() {
    let mut it = Interp::new();
    let result = tool_call(
        &mut it,
        14,
        "calc_eval",
        json!({"expression":"1/2","mode":"frac","digits":5}),
    );
    assert_eq!(result["content"][0]["text"], "1/2");
    assert_eq!(result["structuredContent"]["mode"], "frac");
    // session config must be back to defaults
    let get = tool_call(&mut it, 15, "calc_config", json!({"action":"get"}));
    let sc = &get["structuredContent"];
    assert_eq!(sc["mode"], "real");
    assert_eq!(sc["digits"], 20);
}
