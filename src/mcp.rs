//! MCP server: JSON-RPC 2.0 over stdio.

use crate::builtins;
use crate::config::Mode;
use crate::eval::Interp;
use serde_json::{json, Value as J};
use std::io::{BufRead, Write};

pub const PROTOCOL_VERSION: &str = "2025-06-18";

pub fn tools_list_result() -> J {
    json!({
        "tools": [
            {
                "name": "calc_eval",
                "title": "Evaluate calc expression",
                "description": "Evaluate an arbitrary-precision calc expression (exact rational arithmetic) and return the result.",
                "inputSchema": {
                    "type": "object",
                    "properties": {
                        "expression": {
                            "type": "string",
                            "description": "The expression to evaluate"
                        },
                        "mode": {
                            "type": "string",
                            "enum": ["real", "frac", "int"],
                            "description": "Output rendering (default: real)"
                        },
                        "digits": {
                            "type": "integer",
                            "minimum": 1,
                            "maximum": 10000,
                            "description": "Decimal digits in real mode (default 20)"
                        },
                        "epsilon": {
                            "type": "string",
                            "description": "Precision target for irrational results"
                        }
                    },
                    "required": ["expression"],
                    "additionalProperties": false
                }
            },
            {
                "name": "calc_config",
                "title": "Get or set session configuration",
                "description": "Read or update persistent session configuration",
                "inputSchema": {
                    "type": "object",
                    "properties": {
                        "action": {
                            "type": "string",
                            "enum": ["get", "set"],
                            "description": "'get' returns current config; 'set' updates fields"
                        },
                        "mode":    { "type": "string", "enum": ["real", "frac", "int"] },
                        "digits":  { "type": "integer", "minimum": 1, "maximum": 10000 },
                        "epsilon": { "type": "string" }
                    },
                    "required": ["action"],
                    "additionalProperties": false
                }
            },
            {
                "name": "calc_functions",
                "title": "List builtin functions",
                "description": "List available builtin functions, optionally filtered",
                "inputSchema": {
                    "type": "object",
                    "properties": {
                        "filter": {
                            "type": "string",
                            "description": "Optional case-insensitive substring filter"
                        }
                    },
                    "additionalProperties": false
                }
            },
            {
                "name": "calc_session",
                "title": "Session control",
                "description": "Reset session or get session state",
                "inputSchema": {
                    "type": "object",
                    "properties": {
                        "action": {
                            "type": "string",
                            "enum": ["reset", "state"],
                            "description": "'reset' clears all variables and restores defaults; 'state' shows current session info"
                        }
                    },
                    "required": ["action"],
                    "additionalProperties": false
                }
            }
        ]
    })
}

fn server_info() -> J {
    json!({
        "protocolVersion": PROTOCOL_VERSION,
        "capabilities": { "tools": { "listChanged": false } },
        "serverInfo": {
            "name": "toRustCalcMCP",
            "version": crate::RCALC_VERSION,
            "title": "calc — arbitrary-precision calculator"
        },
        "instructions": "Use calc_eval for arithmetic. Numbers are exact rationals; irrational results honour epsilon."
    })
}

fn ok_text(id: J, text: String, is_error: bool) -> J {
    json!({
        "jsonrpc": "2.0",
        "id": id,
        "result": {
            "content": [ { "type": "text", "text": text } ],
            "isError": is_error
        }
    })
}

fn ok_result(id: J, result: J) -> J {
    json!({ "jsonrpc": "2.0", "id": id, "result": result })
}

fn err(id: J, code: i64, message: &str) -> J {
    json!({ "jsonrpc": "2.0", "id": id, "error": { "code": code, "message": message } })
}

fn handle_tool_call(it: &mut Interp, params: &J) -> J {
    let name = params.get("name").and_then(|v| v.as_str()).unwrap_or("");
    let args = params.get("arguments").cloned().unwrap_or(json!({}));
    match name {
        "calc_eval" => {
            let Some(expr) = args.get("expression").and_then(|v| v.as_str()) else {
                return json!({ "content": [{ "type": "text", "text": "calc_eval: missing 'expression'" }], "isError": true });
            };
            let saved = it.cfg.clone();
            if let Some(m) = args.get("mode").and_then(|v| v.as_str()) {
                if let Some(mode) = Mode::parse(m) {
                    it.cfg.mode = mode;
                }
            }
            if let Some(d) = args.get("digits").and_then(|v| v.as_u64()) {
                it.cfg.display = d as usize;
            }
            if let Some(e) = args.get("epsilon").and_then(|v| v.as_str()) {
                let _ = it.cfg.set_epsilon_from_str(e);
            }
            let res = it.eval_render(expr);
            it.cfg = saved;
            match res {
                Ok(text) => {
                    let result_json = json!({
                        "expression": expr,
                        "result": text,
                        "mode": it.cfg.mode.as_str()
                    });
                    json!({
                        "content": [
                            { "type": "text", "text": text },
                            { "type": "application/json", "json": result_json }
                        ],
                        "isError": false
                    })
                }
                Err(e) => {
                    let error_json = json!({
                        "expression": expr,
                        "error": e
                    });
                    json!({
                        "content": [
                            { "type": "text", "text": format!("error: {e}") },
                            { "type": "application/json", "json": error_json }
                        ],
                        "isError": true
                    })
                }
            }
        }
        "calc_config" => {
            let action = args.get("action").and_then(|v| v.as_str()).unwrap_or("get");
            if action == "set" {
                if let Some(m) = args.get("mode").and_then(|v| v.as_str()) {
                    if let Some(mode) = Mode::parse(m) {
                        it.cfg.mode = mode;
                    }
                }
                if let Some(d) = args.get("digits").and_then(|v| v.as_u64()) {
                    it.cfg.display = d as usize;
                }
                if let Some(e) = args.get("epsilon").and_then(|v| v.as_str()) {
                    if let Err(msg) = it.cfg.set_epsilon_from_str(e) {
                        return json!({ "content": [{ "type": "text", "text": msg }], "isError": true });
                    }
                }
                if let Some(i) = args.get("ibase").and_then(|v| v.as_u64()) {
                    let ib = i as u32;
                    if ib >= 2 && ib <= 36 {
                        it.cfg.ibase = ib;
                    }
                }
                if let Some(o) = args.get("obase").and_then(|v| v.as_u64()) {
                    let ob = o as u32;
                    if ob >= 2 && ob <= 36 {
                        it.cfg.obase = ob;
                    }
                }
            }
            // Return both text and structured JSON
            let config_json = json!({
                "mode": it.cfg.mode.as_str(),
                "digits": it.cfg.display,
                "epsilon": crate::number::to_decimal_string(&it.cfg.epsilon, 60),
                "ibase": it.cfg.ibase,
                "obase": it.cfg.obase
            });
            let text = format!(
                "mode={} digits={} epsilon={} ibase={} obase={}",
                it.cfg.mode.as_str(),
                it.cfg.display,
                crate::number::to_decimal_string(&it.cfg.epsilon, 60),
                it.cfg.ibase,
                it.cfg.obase
            );
            json!({
                "content": [
                    { "type": "text", "text": text },
                    { "type": "application/json", "json": config_json }
                ],
                "isError": false
            })
        }
        "calc_functions" => {
            let filter = args
                .get("filter")
                .and_then(|v| v.as_str())
                .map(|s| s.to_ascii_lowercase());
            let mut lines = Vec::new();
            let mut functions = Vec::new();
            for (nm, sig, desc) in builtins::catalog() {
                if let Some(f) = &filter {
                    if !nm.to_ascii_lowercase().contains(f.as_str()) {
                        continue;
                    }
                }
                lines.push(format!("{sig:<16} {desc}"));
                functions.push(json!({
                    "name": nm,
                    "signature": sig,
                    "description": desc
                }));
            }
            let text = if lines.is_empty() {
                "no matching functions".to_string()
            } else {
                lines.join("\n")
            };
            let functions_json = json!({
                "count": functions.len(),
                "functions": functions
            });
            json!({
                "content": [
                    { "type": "text", "text": text },
                    { "type": "application/json", "json": functions_json }
                ],
                "isError": false
            })
        }
        "calc_session" => {
            let action = args.get("action").and_then(|v| v.as_str()).unwrap_or("state");
            if action == "reset" {
                *it = Interp::new();
                json!({ "content": [{ "type": "text", "text": "session reset" }], "isError": false })
            } else {
                let state_json = json!({
                    "variables": it.global_vars.len(),
                    "scopes": it.scope_stack.len(),
                    "mode": it.cfg.mode.as_str(),
                    "ibase": it.cfg.ibase,
                    "obase": it.cfg.obase,
                    "epsilon": crate::number::to_decimal_string(&it.cfg.epsilon, 60)
                });
                let text = format!(
                    "variables={} scopes={} mode={} ibase={} obase={} epsilon={}",
                    it.global_vars.len(),
                    it.scope_stack.len(),
                    it.cfg.mode.as_str(),
                    it.cfg.ibase,
                    it.cfg.obase,
                    crate::number::to_decimal_string(&it.cfg.epsilon, 60)
                );
                json!({
                    "content": [
                        { "type": "text", "text": text },
                        { "type": "application/json", "json": state_json }
                    ],
                    "isError": false
                })
            }
        }
        other => {
            json!({ "content": [{ "type": "text", "text": format!("unknown tool: {other}") }], "isError": true })
        }
    }
}

pub fn handle_message(it: &mut Interp, msg: &J) -> Option<J> {
    let id = msg.get("id").cloned();
    let method = msg.get("method").and_then(|v| v.as_str()).unwrap_or("");
    let params = msg.get("params").cloned().unwrap_or(json!({}));

    let is_notification = id.is_none();

    match method {
        "initialize" => Some(ok_result(id.unwrap_or(J::Null), server_info())),
        "notifications/initialized" | "initialized" => None,
        "ping" => Some(ok_result(id.unwrap_or(J::Null), json!({}))),
        "tools/list" => Some(ok_result(id.unwrap_or(J::Null), tools_list_result())),
        "tools/call" => {
            let result = handle_tool_call(it, &params);
            Some(ok_result(id.unwrap_or(J::Null), result))
        }
        _ if is_notification => None,
        _ => Some(err(id.unwrap_or(J::Null), -32601, "method not found")),
    }
}

pub fn serve_stdio() -> std::io::Result<()> {
    let stdin = std::io::stdin();
    let stdout = std::io::stdout();
    let mut out = stdout.lock();
    let mut it = Interp::new();

    for line in stdin.lock().lines() {
        let line = line?;
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }
        let parsed: Result<J, _> = serde_json::from_str(trimmed);
        let response = match parsed {
            Ok(msg) => handle_message(&mut it, &msg),
            Err(e) => Some(err(J::Null, -32700, &format!("parse error: {e}"))),
        };
        if let Some(resp) = response {
            writeln!(out, "{}", serde_json::to_string(&resp).unwrap())?;
            out.flush()?;
        }
    }
    Ok(())
}
