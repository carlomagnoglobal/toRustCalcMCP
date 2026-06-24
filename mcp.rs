//! MCP server: JSON-RPC 2.0 over stdio (newline-delimited messages).
//!
//! Implements the handshake (`initialize` / `notifications/initialized`), tool
//! discovery (`tools/list`), invocation (`tools/call`), and `ping`. Tools:
//!
//!   * calc_eval      — evaluate a calc expression
//!   * calc_config    — get/set session precision & display
//!   * calc_functions — list available builtin functions
//!
//! The full tool schema is produced by `tools_list_result()` and is identical to
//! the standalone schema document shipped with this project.

use crate::builtins;
use crate::config::Mode;
use crate::eval::Interp;
use serde_json::{json, Value as J};
use std::io::{BufRead, Write};

pub const PROTOCOL_VERSION: &str = "2025-06-18";

/// JSON Schema for every tool, as the `tools/list` result payload.
pub fn tools_list_result() -> J {
    json!({
        "tools": [
            {
                "name": "calc_eval",
                "title": "Evaluate calc expression",
                "description": "Evaluate an arbitrary-precision calc expression (exact rational arithmetic) and return the result. Supports + - * / // % ^, comparisons, variables, and builtin functions such as sqrt, gcd, fact, isprime, sin, pi.",
                "inputSchema": {
                    "type": "object",
                    "properties": {
                        "expression": {
                            "type": "string",
                            "description": "The expression to evaluate, e.g. \"2^100 + 1\" or \"sqrt(2)\" or \"gcd(462,1071)\". Multiple statements may be separated by ';'."
                        },
                        "mode": {
                            "type": "string",
                            "enum": ["real", "frac", "int"],
                            "description": "Output rendering: 'real' decimal (default), 'frac' exact a/b, or 'int' truncated integer. Applies to this call only."
                        },
                        "digits": {
                            "type": "integer",
                            "minimum": 1,
                            "maximum": 10000,
                            "description": "Decimal digits shown in real mode (default 20). This call only."
                        },
                        "epsilon": {
                            "type": "string",
                            "description": "Precision target for irrational results, e.g. \"1e-40\". This call only."
                        }
                    },
                    "required": ["expression"],
                    "additionalProperties": false
                }
            },
            {
                "name": "calc_config",
                "title": "Get or set session configuration",
                "description": "Read or update the persistent session configuration (precision epsilon, display digits, output mode).",
                "inputSchema": {
                    "type": "object",
                    "properties": {
                        "action": {
                            "type": "string",
                            "enum": ["get", "set"],
                            "description": "'get' returns current config; 'set' updates one or more fields."
                        },
                        "mode":    { "type": "string", "enum": ["real", "frac", "int"] },
                        "digits":  { "type": "integer", "minimum": 1, "maximum": 10000 },
                        "epsilon": { "type": "string", "description": "e.g. \"1e-20\"" }
                    },
                    "required": ["action"],
                    "additionalProperties": false
                }
            },
            {
                "name": "calc_functions",
                "title": "List builtin functions",
                "description": "List the available builtin functions with signatures and descriptions, optionally filtered by a substring.",
                "inputSchema": {
                    "type": "object",
                    "properties": {
                        "filter": {
                            "type": "string",
                            "description": "Optional case-insensitive substring to match against function names."
                        }
                    },
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
        "instructions": "Use calc_eval for arithmetic. Numbers are exact rationals; irrational results honour the session epsilon. Use calc_config to change precision/display, and calc_functions to discover builtins."
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
            // per-call overrides
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
            it.cfg = saved; // overrides are per-call
            match res {
                Ok(text) => json!({ "content": [{ "type": "text", "text": text }], "isError": false }),
                Err(e) => json!({ "content": [{ "type": "text", "text": format!("error: {e}") }], "isError": true }),
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
            }
            let text = format!(
                "mode={} digits={} epsilon={}",
                it.cfg.mode.as_str(),
                it.cfg.display,
                crate::number::to_decimal_string(&it.cfg.epsilon, 60)
            );
            json!({ "content": [{ "type": "text", "text": text }], "isError": false })
        }
        "calc_functions" => {
            let filter = args
                .get("filter")
                .and_then(|v| v.as_str())
                .map(|s| s.to_ascii_lowercase());
            let mut lines = Vec::new();
            for (nm, sig, desc) in builtins::catalog() {
                if let Some(f) = &filter {
                    if !nm.to_ascii_lowercase().contains(f.as_str()) {
                        continue;
                    }
                }
                lines.push(format!("{sig:<16} {desc}"));
            }
            let text = if lines.is_empty() {
                "no matching functions".to_string()
            } else {
                lines.join("\n")
            };
            json!({ "content": [{ "type": "text", "text": text }], "isError": false })
        }
        other => {
            json!({ "content": [{ "type": "text", "text": format!("unknown tool: {other}") }], "isError": true })
        }
    }
}

/// Dispatch a single JSON-RPC request object. Returns `Some(response)` for
/// requests, or `None` for notifications (which get no reply).
pub fn handle_message(it: &mut Interp, msg: &J) -> Option<J> {
    let id = msg.get("id").cloned();
    let method = msg.get("method").and_then(|v| v.as_str()).unwrap_or("");
    let params = msg.get("params").cloned().unwrap_or(json!({}));

    // Notifications have no id and expect no response.
    let is_notification = id.is_none();

    match method {
        "initialize" => Some(ok_result(id.unwrap_or(J::Null), server_info())),
        "notifications/initialized" | "initialized" => None,
        "ping" => Some(ok_result(id.unwrap_or(J::Null), json!({}))),
        "tools/list" => Some(ok_result(id.unwrap_or(J::Null), tools_list_result())),
        "tools/call" => {
            let result = handle_tool_call(it, &params);
            let is_err = result.get("isError").and_then(|v| v.as_bool()).unwrap_or(false);
            let text = result
                .get("content")
                .and_then(|c| c.get(0))
                .and_then(|c| c.get("text"))
                .and_then(|t| t.as_str())
                .unwrap_or("")
                .to_string();
            Some(ok_text(id.unwrap_or(J::Null), text, is_err))
        }
        _ if is_notification => None,
        _ => Some(err(id.unwrap_or(J::Null), -32601, "method not found")),
    }
}

/// Run the stdio server loop until EOF.
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
