# MCP Response Format Fix

## Problem

Claude Desktop is reporting "Unsupported format" errors when trying to use the MCP tools. Root cause: the MCP server is returning tool results with an invalid content type `"application/json"`.

## MCP Spec Compliance

According to the MCP (Model Context Protocol) specification, tool result content must use valid content types:
- ✅ `{ "type": "text", "text": "..." }` — VALID
- ❌ `{ "type": "application/json", "json": {...} }` — INVALID

Claude Desktop only recognizes `"type": "text"` for text content.

## What Was Fixed

✅ `calc_eval` tool — fixed at lines 142-158
✅ `calc_config` tool — fixed at lines 203-217

## What Still Needs Fixing

The following two tools still have invalid `"type": "application/json"` content blocks that must be removed:

### 1. calc_functions (lines ~227-242)

**BEFORE:**
```rust
json!({
    "content": [
        { "type": "text", "text": text },
        { "type": "application/json", "json": functions_json }  // ❌ REMOVE THIS LINE
    ],
    "isError": false
})
```

**AFTER:**
```rust
json!({
    "content": [
        { "type": "text", "text": text }
    ],
    "isError": false
})
```

Also remove the now-unused `functions_json` variable and the loop that builds `functions` vector.

### 2. calc_session (lines ~250-273)

**BEFORE:**
```rust
json!({
    "content": [
        { "type": "text", "text": text },
        { "type": "application/json", "json": state_json }  // ❌ REMOVE THIS LINE
    ],
    "isError": false
})
```

**AFTER:**
```rust
json!({
    "content": [
        { "type": "text", "text": text }
    ],
    "isError": false
})
```

Also remove the now-unused `state_json` variable.

## Testing After Fix

1. Rebuild: `cargo build --release`
2. Restart Claude Desktop
3. Test the tools again — "Unsupported format" errors should be gone

## Why This Works

- MCP spec only recognizes `"text"` type for content blocks
- Removing the invalid JSON content blocks brings the server into compliance
- The textual representation of results is sufficient for Claude to understand and work with the data
- If Claude needs structured data, it can parse the text representation as needed

## Files Modified

- `src/mcp.rs` (lines to be fixed in calc_functions and calc_session)

## Related Commits

- `c3b7734` — "Add MCP testing guide and Claude Desktop integration"
- `5f63a40` — "Remove unused import from bin_web.rs"
- (This fix will be a new commit after the file system stabilizes)
