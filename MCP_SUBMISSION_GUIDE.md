# MCP.SO Submission Configuration Guide

This guide shows how to configure toRustCalcMCP for submission to the MCP registry at https://mcp.so/submit

---

## MCP Server Information

**Name:** toRustCalcMCP  
**Repository:** https://github.com/carlomagnoglobal/toRustCalcMCP  
**Description:** Rust port of calc - exact-rational calculator and arbitrary-precision math engine deployed as an MCP server

---

## Server Configuration (for mcp.so)

Choose **ONE** of the three configurations below based on your deployment preference:

### Configuration 1: Pre-built Binary (Recommended)

**Best for:** End users who want the simplest setup

```json
{
  "mcpServers": {
    "toRustCalcMCP": {
      "command": "/usr/local/bin/toRustCalcMCP",
      "args": ["--mcp"]
    }
  }
}
```

**Installation Instructions:**
```bash
# Download the latest binary for your platform from:
# https://github.com/carlomagnoglobal/toRustCalcMCP/releases

# macOS (Apple Silicon):
curl -L https://github.com/carlomagnoglobal/toRustCalcMCP/releases/download/latest/rcalc-aarch64-apple-darwin.tar.gz \
  | tar xz -C /usr/local/bin --strip-components=1

# macOS (Intel):
curl -L https://github.com/carlomagnoglobal/toRustCalcMCP/releases/download/latest/rcalc-x86_64-apple-darwin.tar.gz \
  | tar xz -C /usr/local/bin --strip-components=1

# Linux (x86_64):
curl -L https://github.com/carlomagnoglobal/toRustCalcMCP/releases/download/latest/rcalc-linux-x86_64.tar.gz \
  | tar xz -C /usr/local/bin --strip-components=1

# Make executable
chmod +x /usr/local/bin/toRustCalcMCP
```

---

### Configuration 2: From Source (Cargo)

**Best for:** Developers who want the latest code and full transparency

```json
{
  "mcpServers": {
    "toRustCalcMCP": {
      "command": "cargo",
      "args": [
        "run",
        "--release",
        "--bin",
        "toRustCalcMCP",
        "--",
        "--mcp"
      ],
      "cwd": "/path/to/toRustCalcMCP"
    }
  }
}
```

**Installation Instructions:**
```bash
# Install Rust (if not already installed):
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env

# Clone and setup
git clone https://github.com/carlomagnoglobal/toRustCalcMCP.git
cd toRustCalcMCP

# Build (optional - cargo run will build if needed)
cargo build --release
```

**Update Instructions:**
```bash
cd /path/to/toRustCalcMCP
git pull
# That's it - cargo run will rebuild if needed
```

---

### Configuration 3: Docker

**Best for:** Containerized/team environments with strict isolation requirements

```json
{
  "mcpServers": {
    "toRustCalcMCP": {
      "command": "docker",
      "args": [
        "run",
        "-i",
        "--rm",
        "ghcr.io/carlomagnoglobal/toRustCalcMCP:latest"
      ]
    }
  }
}
```

**Installation Instructions:**
```bash
# Pull the latest image
docker pull ghcr.io/carlomagnoglobal/toRustCalcMCP:latest

# Test it works
docker run -i --rm ghcr.io/carlomagnoglobal/toRustCalcMCP:latest <<'EOF'
{"jsonrpc":"2.0","id":1,"method":"initialize","params":{}}
{"jsonrpc":"2.0","id":2,"method":"tools/list"}
EOF
```

**Update Instructions:**
```bash
docker pull ghcr.io/carlomagnoglobal/toRustCalcMCP:latest
```

---

## For mcp.so Submission Form

Fill in the form at https://mcp.so/submit with:

| Field | Value |
|-------|-------|
| **MCP Server Name** | toRustCalcMCP |
| **URL/Repository** | https://github.com/carlomagnoglobal/toRustCalcMCP |
| **Description** | Exact-rational arbitrary-precision calculator as an MCP server. Implements 351 mathematical functions including transcendentals, special functions, complex numbers, list operations, file I/O, and more. Rust port of the classic calc language. |
| **Categories** | Calculator, Math, Utility |
| **Server Config** | *Choose one from above* |

---

## Quick Copy-Paste Configs

### Option 1 (Binary) - Copy This:
```json
{
  "mcpServers": {
    "toRustCalcMCP": {
      "command": "/usr/local/bin/toRustCalcMCP",
      "args": ["--mcp"]
    }
  }
}
```

### Option 2 (Cargo) - Copy This:
```json
{
  "mcpServers": {
    "toRustCalcMCP": {
      "command": "cargo",
      "args": ["run", "--release", "--bin", "toRustCalcMCP", "--", "--mcp"],
      "cwd": "/path/to/toRustCalcMCP"
    }
  }
}
```

### Option 3 (Docker) - Copy This:
```json
{
  "mcpServers": {
    "toRustCalcMCP": {
      "command": "docker",
      "args": ["run", "-i", "--rm", "ghcr.io/carlomagnoglobal/toRustCalcMCP:latest"]
    }
  }
}
```

---

## Features to Highlight

✅ **351 Mathematical Functions** - Complete calc language compatibility  
✅ **Exact Rational Arithmetic** - No floating point errors  
✅ **Arbitrary Precision** - Compute to millions of digits  
✅ **Transcendental Functions** - sin, cos, tan, log, exp, and 100+ more  
✅ **Special Functions** - Bessel, Gamma, Zeta, Error functions  
✅ **Complex Numbers** - Full complex arithmetic support  
✅ **List Operations** - sort, reverse, unique, flatten, zip, etc.  
✅ **File I/O** - Read/write files with full formatting support  
✅ **String Operations** - substr, split, replace, trim, padding  
✅ **Matrix Operations** - multiply, transpose, determinant, inverse  
✅ **System Integration** - Environment variables, file system, process info  
✅ **Fast Startup** - Minimal binary (Option 1: ~50MB)  
✅ **No External Dependencies** - Pure Rust implementation  
✅ **Well-Tested** - 359 integration tests, all passing  

---

## Testing Your Installation

After installation, test with any of these commands:

**Test 1: Basic Arithmetic**
```bash
echo '{"jsonrpc":"2.0","id":1,"method":"initialize","params":{}}' | toRustCalcMCP --mcp
```

**Test 2: Exact Calculation**
```bash
echo '{"jsonrpc":"2.0","id":1,"method":"tools/call","params":{"name":"calc_eval","arguments":{"expression":"1/3 + 1/6"}}}' | toRustCalcMCP --mcp
```

**Test 3: Large Number**
```bash
echo '{"jsonrpc":"2.0","id":1,"method":"tools/call","params":{"name":"calc_eval","arguments":{"expression":"2^256"}}}' | toRustCalcMCP --mcp
```

---

## Support & Documentation

- **Installation Help:** See [DEPLOYMENT.md](DEPLOYMENT.md)
- **Quick Start:** See [QUICKSTART.md](QUICKSTART.md)
- **Full Usage:** See [README.md](README.md)
- **Issue Tracker:** https://github.com/carlomagnoglobal/toRustCalcMCP/issues
- **Discussions:** https://github.com/carlomagnoglobal/toRustCalcMCP/discussions

---

## Environment Requirements

| Requirement | Option 1 | Option 2 | Option 3 |
|-------------|----------|----------|----------|
| Disk Space | ~50 MB | ~1.5 GB | ~20 GB |
| Runtime | None | Rust 1.75+ | Docker |
| Network | No | Yes (during build) | Yes (during pull) |
| Setup Time | 2 min | 5-10 min | 5-15 min |

---

## Troubleshooting

**"Command not found"** (Option 1)
- Ensure binary is in PATH: `which toRustCalcMCP`
- Or use full path: `/usr/local/bin/toRustCalcMCP`

**"cargo: command not found"** (Option 2)
- Install Rust: https://rustup.rs/

**"docker: command not found"** (Option 3)
- Install Docker: https://www.docker.com/products/docker-desktop

**"Permission denied"** (Option 1)
- Run: `chmod +x /usr/local/bin/toRustCalcMCP`

---

Generated: 2025-06-30  
For latest updates: https://github.com/carlomagnoglobal/toRustCalcMCP/releases
