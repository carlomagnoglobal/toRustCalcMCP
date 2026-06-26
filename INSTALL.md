# Installation & Setup

## Quick Start — Claude Desktop Integration

### Step 1: Build the binary

```sh
cargo build --release
```

The MCP binary will be at `./target/release/toRustCalcMCP`.

### Step 2: Get the absolute path

```sh
pwd
# Copy the output, e.g., /Users/yourname/Documents/toRustCalcMCP
```

### Step 3: Update Claude Desktop config

Edit `~/Library/Application Support/Claude/claude_desktop_config.json`:

```json
{
  "mcpServers": {
    "calc": {
      "command": "/Users/yourname/Documents/toRustCalcMCP/target/release/toRustCalcMCP",
      "args": ["--mcp"]
    }
  }
}
```

Replace `/Users/yourname/Documents/toRustCalcMCP` with your actual path from Step 2.

### Step 4: Restart Claude Desktop

Close and reopen Claude Desktop, or kill and restart the process:

```sh
pkill -f "Claude"  # kill all Claude processes
# Then reopen Claude.app from Applications
```

### Step 5: Verify

Open Claude Desktop and look for the **"calc" tool** in the Models panel on the left. You should see it listed under **Available Tools** with 4 sub-tools:
- `calc_eval`
- `calc_config`
- `calc_functions`
- `calc_session`

Try asking Claude to "calculate 2^256" or "what is sin(pi/6)?" — it should use the calc tool.

---

## Alternative: Use the example config

An example config template is provided at `examples/mcp-config.json`. You can copy and modify it:

```sh
cp examples/mcp-config.json ~/Library/Application\ Support/Claude/claude_desktop_config.json
# Edit the file and replace the path with your actual binary path
```

---

## Using the CLI (`rcalc`)

You don't need the MCP setup to use the calculator as a command-line tool:

```sh
cargo build --release
./target/release/rcalc '2^100'
./target/release/rcalc -m frac '1/3 + 1/6'
./target/release/rcalc                    # interactive REPL
```

Or symlink for convenience:

```sh
ln -s target/release/toRustCalcMCP target/release/rcalc
./target/release/rcalc 'gcd(462,1071)'
```

---

## Using the Web REPL

```sh
cargo build --release
./target/release/rcalc-web
# Open http://localhost:8888 in your browser
```

---

## Troubleshooting

### "calc tool not found in Claude Desktop"

1. Verify the path in `claude_desktop_config.json` is absolute and correct.
2. Check that the binary exists: `ls -la /path/to/target/release/toRustCalcMCP`
3. Test the binary directly:
   ```sh
   ./target/release/toRustCalcMCP --mcp
   # Should print JSON-RPC version and server info, then wait for input
   ```
4. Restart Claude Desktop.

### "invalid path" or similar error

Ensure the path in the config is absolute (starts with `/`) and uses forward slashes, not backslashes.

### Tool calls fail with an error message

Test the MCP server directly with a JSON-RPC call:

```sh
echo '{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":"2024-11-05","capabilities":{},"clientInfo":{"name":"test","version":"1.0"}}}' | \
  ./target/release/toRustCalcMCP --mcp
```

If this succeeds, the server is working. The issue may be in Claude Desktop's integration. Try restarting Claude Desktop again.

---

## For developers

See [README.md](README.md) for language syntax, [GETTING_STARTED.md](GETTING_STARTED.md) for examples, and [docs/MCP_TOOL_SCHEMA.json](docs/MCP_TOOL_SCHEMA.json) for the authoritative MCP tool schema.
