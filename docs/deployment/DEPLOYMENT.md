# toRustCalcMCP Deployment Guide

This document describes three ways to deploy and use the toRustCalcMCP MCP server.

---

## Option 1: Pre-built Binaries

### For macOS & Linux Users

Download pre-built binaries from [GitHub Releases](https://github.com/carlomagnoglobal/toRustCalcMCP/releases).

**Installation:**
```bash
# Download the appropriate binary for your platform
# macOS (ARM64):
curl -L https://github.com/carlomagnoglobal/toRustCalcMCP/releases/download/v0.1.0/toRustCalcMCP-aarch64-apple-darwin -o toRustCalcMCP
chmod +x toRustCalcMCP

# macOS (Intel):
curl -L https://github.com/carlomagnoglobal/toRustCalcMCP/releases/download/v0.1.0/toRustCalcMCP-x86_64-apple-darwin -o toRustCalcMCP
chmod +x toRustCalcMCP

# Linux (x86_64):
curl -L https://github.com/carlomagnoglobal/toRustCalcMCP/releases/download/v0.1.0/toRustCalcMCP-x86_64-unknown-linux-gnu -o toRustCalcMCP
chmod +x toRustCalcMCP
```

**Usage in Claude configuration:**
```json
{
  "mcpServers": {
    "toRustCalcMCP": {
      "command": "/path/to/toRustCalcMCP",
      "args": ["--mcp"]
    }
  }
}
```

**Advantages:**
- ✅ No Rust or Docker required
- ✅ Fastest startup time
- ✅ Smallest footprint
- ✅ Works offline after download

---

## Option 2: Cargo (Source Build)

### For Rust Developers

If you have Rust installed, build directly from source:

**Installation:**
```bash
# Install Rust if you don't have it:
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Clone and build
git clone https://github.com/carlomagnoglobal/toRustCalcMCP.git
cd toRustCalcMCP
cargo build --release
```

The binary will be at `target/release/toRustCalcMCP`.

**Usage in Claude configuration:**
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

**Advantages:**
- ✅ Latest features
- ✅ Full source transparency
- ✅ Easy to modify and extend
- ✅ Works on any platform with Rust

**Disadvantages:**
- ❌ Requires Rust toolchain (~1.5GB)
- ❌ Slower startup (compilation on first run)

---

## Option 3: Docker

### For Containerized Deployment

**Pull the image:**
```bash
docker pull ghcr.io/carlomagnoglobal/toRustCalcMCP:latest
```

Or build locally:
```bash
git clone https://github.com/carlomagnoglobal/toRustCalcMCP.git
cd toRustCalcMCP
docker build -t toRustCalcMCP:latest .
```

**Usage in Claude configuration:**
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

**Advantages:**
- ✅ Completely isolated environment
- ✅ No system dependencies needed
- ✅ Consistent across all platforms
- ✅ Easy to version and update

**Disadvantages:**
- ❌ Requires Docker (~20GB for full installation)
- ❌ Slightly slower startup (container overhead)
- ❌ Uses more resources

---

## Quick Comparison

| Feature | Option 1 | Option 2 | Option 3 |
|---------|----------|----------|----------|
| **Setup Time** | 2 min | 5-10 min | 5-15 min |
| **Runtime Speed** | ⚡⚡⚡ | ⚡⚡ | ⚡ |
| **Dependencies** | None | Rust | Docker |
| **Disk Space** | ~50MB | ~1.5GB+ | ~20GB+ |
| **Latest Features** | Stable release | ✅ Always latest | Latest |
| **Easy Updates** | Manual download | `git pull` + rebuild | `docker pull` |
| **Recommended For** | Most users | Developers | Teams/CI |

---

## Installation Steps by Use Case

### I just want to use it
→ **Use Option 1 (Pre-built Binary)**
```bash
# Download binary once, use forever
```

### I want the latest features
→ **Use Option 2 (Cargo)**
```bash
cargo run --release --bin toRustCalcMCP -- --mcp
```

### I'm in a containerized environment
→ **Use Option 3 (Docker)**
```bash
docker run -i --rm ghcr.io/carlomagnoglobal/toRustCalcMCP:latest
```

---

## Troubleshooting

### Option 1 Issues
- **"Permission denied"** → Run `chmod +x toRustCalcMCP`
- **"Binary not found for platform"** → Check your CPU architecture: `uname -m`
- **"Segmentation fault"** → Try Option 2 or 3

### Option 2 Issues
- **"cargo not found"** → Install Rust: `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
- **Compilation error** → Ensure Rust 1.75+: `rustc --version`

### Option 3 Issues
- **"docker: command not found"** → Install Docker Desktop
- **Connection refused** → Ensure Docker daemon is running: `docker ps`
- **Out of disk space** → Docker needs ~20GB free

---

## Next Steps

1. Choose your preferred deployment option
2. Follow the installation steps above
3. Add to your Claude configuration
4. Test with: `echo '{"jsonrpc":"2.0","id":1,"method":"initialize","params":{}}' | toRustCalcMCP --mcp`

For more information, see [README.md](../../README.md) and [QUICKSTART.md](QUICKSTART.md).
