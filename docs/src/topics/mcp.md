MCP SERVER MODE

rcalc can run as a JSON-RPC 2.0 MCP server for integration with AI tools.

STARTING THE SERVER

  rcalc --mcp

The server reads JSON-RPC 2.0 requests from stdin and writes responses to stdout.

PROTOCOL

Requests are newline-delimited JSON. Example:

  {"jsonrpc":"2.0","id":1,"method":"initialize","params":{}}

Response:

  {"jsonrpc":"2.0","id":1,"result":{"protocolVersion":"2025-06-18",...}}

INITIALIZATION

First, initialize the connection:

  {"jsonrpc":"2.0","id":1,"method":"initialize","params":{}}

Response includes protocol version, server info, and capabilities.

TOOLS

The server provides 4 tools via tools/call:

  1. calc_eval — Evaluate an expression
  2. calc_config — Get or set session configuration
  3. calc_functions — List builtin functions
  4. calc_session — Reset or inspect session state

TOOL: CALC_EVAL

Evaluate an arbitrary-precision expression:

  {"jsonrpc":"2.0","id":3,"method":"tools/call",
   "params":{"name":"calc_eval",
            "arguments":{"expression":"2^256",
                        "mode":"real",
                        "epsilon":"1e-20"}}}

Response includes a text block plus machine-readable structuredContent:

  {"jsonrpc":"2.0","id":3,"result":{
    "content":[{"type":"text","text":"<numeric result>"}],
    "structuredContent":{"expression":"2^256","result":"<numeric result>","mode":"real"},
    "isError":false}}

All four tools return structuredContent on success (shape declared in each
tool's outputSchema, visible via tools/list). Errors return only a text
block with isError:true.

Arguments:
  - expression (required): expression string
  - mode (optional): real, frac, or int
  - epsilon (optional): precision string (e.g., "1e-50")
  - digits (optional): display width (e.g., 50)

TOOL: CALC_CONFIG

Get or set session configuration:

  {"jsonrpc":"2.0","id":4,"method":"tools/call",
   "params":{"name":"calc_config",
            "arguments":{"action":"get"}}}

  {"jsonrpc":"2.0","id":5,"method":"tools/call",
   "params":{"name":"calc_config",
            "arguments":{"action":"set","mode":"frac","epsilon":"1e-50"}}}

Arguments for "set":
  - mode: real, frac, or int
  - epsilon: precision string
  - digits: display width
  - ibase: input base (2-36)
  - obase: output base (2-36)

TOOL: CALC_FUNCTIONS

List available builtin functions, optionally filtered:

  {"jsonrpc":"2.0","id":6,"method":"tools/call",
   "params":{"name":"calc_functions",
            "arguments":{"filter":"sin"}}}

Returns a list of functions matching the filter.

TOOL: CALC_SESSION

Reset or inspect the current session:

  {"jsonrpc":"2.0","id":7,"method":"tools/call",
   "params":{"name":"calc_session",
            "arguments":{"action":"reset"}}}

  {"jsonrpc":"2.0","id":8,"method":"tools/call",
   "params":{"name":"calc_session",
            "arguments":{"action":"state"}}}

Arguments:
  - action: "reset" (clear all variables) or "state" (show current state)

EXAMPLE SESSION

  $ rcalc --mcp

  {"jsonrpc":"2.0","id":1,"method":"initialize","params":{}}
  {"jsonrpc":"2.0","id":2,"method":"tools/list"}
  {"jsonrpc":"2.0","id":3,"method":"tools/call","params":{"name":"calc_eval","arguments":{"expression":"2^256"}}}
  {"jsonrpc":"2.0","id":4,"method":"tools/call","params":{"name":"calc_config","arguments":{"action":"get"}}}

NOTIFICATIONS

Requests without an "id" are notifications; the server does not respond:

  {"jsonrpc":"2.0","method":"notifications/initialized"}