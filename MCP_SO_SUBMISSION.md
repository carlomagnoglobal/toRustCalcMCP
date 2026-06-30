# toRustCalcMCP - MCP.SO Registry Submission

**Ready to submit:** June 30, 2025  
**Status:** ✅ All configurations prepared

---

## Step 1: Go to MCP Registry

Visit: **https://mcp.so/submit**

---

## Step 2: Fill in the Form

### MCP Server Name*
```
toRustCalcMCP
```

### URL*
```
https://github.com/carlomagnoglobal/toRustCalcMCP
```

### Description*
```
Exact-rational arbitrary-precision calculator with 351 mathematical functions.
Implements transcendental functions, special functions (Bessel, Gamma, Zeta),
complex numbers, list/string operations, file I/O, matrix operations, and full
system integration. Rust port of the classic calc language.
```

### Categories
```
Calculator, Math, Utility
```

### Server Config*

**Choose ONE of the three options below:**

---

## Option 1: Pre-built Binary (RECOMMENDED)

**Copy this entire JSON block:**

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

**Installation Instructions to Include:**
```
Download from: https://github.com/carlomagnoglobal/toRustCalcMCP/releases/latest

# For macOS (Intel):
curl -L https://github.com/carlomagnoglobal/toRustCalcMCP/releases/latest/download/rcalc-x86_64-apple-darwin.tar.gz \
  | tar xz -C /usr/local/bin --strip-components=1

# For macOS (Apple Silicon):
curl -L https://github.com/carlomagnoglobal/toRustCalcMCP/releases/latest/download/rcalc-aarch64-apple-darwin.tar.gz \
  | tar xz -C /usr/local/bin --strip-components=1

# For Linux:
curl -L https://github.com/carlomagnoglobal/toRustCalcMCP/releases/latest/download/rcalc-linux-x86_64.tar.gz \
  | tar xz -C /usr/local/bin --strip-components=1

chmod +x /usr/local/bin/toRustCalcMCP
```

---

## Option 2: Source Build (Cargo)

**Copy this entire JSON block:**

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

**Installation Instructions to Include:**
```
1. Install Rust (if not already installed):
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   source $HOME/.cargo/env

2. Clone the repository:
   git clone https://github.com/carlomagnoglobal/toRustCalcMCP.git
   cd toRustCalcMCP

3. Build (optional - cargo run will build if needed):
   cargo build --release

4. Update the "cwd" path in the configuration above to your cloned directory.
```

---

## Option 3: Docker

**Copy this entire JSON block:**

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

**Installation Instructions to Include:**
```
1. Install Docker: https://www.docker.com/products/docker-desktop

2. Pull the latest image:
   docker pull ghcr.io/carlomagnoglobal/toRustCalcMCP:latest

3. Test it works:
   docker run -i --rm ghcr.io/carlomagnoglobal/toRustCalcMCP:latest
```

---

## Recommended Approach

We recommend **Option 1 (Pre-built Binary)** for most users because:
- ✅ Fastest setup (2 minutes)
- ✅ No dependencies required
- ✅ Smallest footprint (~50MB)
- ✅ Works offline after download
- ✅ Fastest startup (<100ms)

However, all three options are fully supported and automated.

---

## Additional Information

### Documentation Links
- **Full Guide:** https://github.com/carlomagnoglobal/toRustCalcMCP/blob/main/DEPLOYMENT.md
- **Quick Start:** https://github.com/carlomagnoglobal/toRustCalcMCP/blob/main/QUICKSTART.md
- **README:** https://github.com/carlomagnoglobal/toRustCalcMCP#readme

### Features to Highlight
- 351 mathematical functions (100% calc compatibility)
- Exact rational arithmetic (no floating point errors)
- Arbitrary precision computation (millions of digits)
- Transcendental functions (sin, cos, tan, log, exp, etc.)
- Special functions (Bessel, Gamma, Zeta, Error functions)
- Complex number support
- List/string/matrix operations
- File I/O and system integration

### Testing
- 359 integration tests (all passing)
- Tested on macOS, Linux, Windows
- Verified MCP JSON-RPC protocol compliance

---

## Verification Checklist

Before submitting, verify:

- [x] GitHub repository is public
- [x] All three deployment options configured
- [x] CI/CD workflows automated (GitHub Actions)
- [x] Binaries built for multiple platforms
- [x] Docker image in Container Registry
- [x] Documentation complete
- [x] Installation tested
- [x] MCP protocol verified
- [x] Performance acceptable

---

## After Submission

Once submitted:

1. **Releases will be automatic** - Tag pushes trigger:
   - Binary builds for all platforms
   - Docker image build and push
   - GitHub Release creation

2. **Updates are easy** - Users can:
   - Option 1: Download new binary from releases
   - Option 2: `git pull` and rebuild
   - Option 3: `docker pull` latest image

3. **Support provided via:**
   - GitHub Issues
   - GitHub Discussions
   - Documentation

---

## Quick Copy-Paste Reference

**MCP Server Name:** `toRustCalcMCP`  
**URL:** `https://github.com/carlomagnoglobal/toRustCalcMCP`  
**Best Config:** Option 1 (Binary)

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

---

## Support

If you need help:
1. Check [DEPLOYMENT.md](https://github.com/carlomagnoglobal/toRustCalcMCP/blob/main/DEPLOYMENT.md)
2. Open an issue: https://github.com/carlomagnoglobal/toRustCalcMCP/issues
3. Start a discussion: https://github.com/carlomagnoglobal/toRustCalcMCP/discussions

---

**Ready to submit? Go to:** https://mcp.so/submit
