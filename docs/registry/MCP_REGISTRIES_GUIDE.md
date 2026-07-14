# Publishing toRustCalcMCP to MCP Registries

Complete guide for submitting to both **mcp.so** and **Docker Hub MCP Registry**.

---

## Registry Overview

| Registry | URL | Best For | Type |
|----------|-----|----------|------|
| **mcp.so** | https://mcp.so/submit | General MCP discovery | Curated registry |
| **Docker Hub MCP** | https://hub.docker.com/mcp | Docker users | Docker-focused |

Both registries point to the same toRustCalcMCP, just using different Docker registries for hosting.

---

## Registry 1: mcp.so (Recommended First)

### Step 1: Visit the Submission Form
Go to: **https://mcp.so/submit**

### Step 2: Fill in the Form

**Field: MCP Server Name***
```
toRustCalcMCP
```

**Field: URL***
```
https://github.com/carlomagnoglobal/toRustCalcMCP
```

**Field: Description***
```
Exact-rational arbitrary-precision calculator with 351 mathematical functions.
Implements transcendental functions, special functions (Bessel, Gamma, Zeta),
complex numbers, list/string operations, file I/O, matrix operations, and full
system integration. Rust port of the classic calc language.

Supports three deployment options:
- Pre-built binaries (recommended)
- Source build with Cargo
- Docker containers (ghcr.io or Docker Hub)
```

**Field: Categories**
```
Calculator, Math, Utility
```

**Field: Server Config*** (Choose one)

### Option A: Pre-built Binary (RECOMMENDED)
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

### Option B: Cargo
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

### Option C: GitHub Container Registry
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

### Step 3: Submit
Click **Submit** or **Create**

---

## Registry 2: Docker Hub MCP Registry

### Prerequisites
1. ✅ Create Docker Hub account: https://hub.docker.com/signup
2. ✅ Create repository: `torustcalcmcp` (public)
3. ✅ Follow DOCKER_HUB_SETUP.md for GitHub Actions integration
4. ✅ Create first release tag to trigger Docker image build

### Step 1: Verify Docker Image is Available
```bash
docker pull carlomagnoglobal/torustcalcmcp:latest
docker run -i --rm carlomagnoglobal/torustcalcmcp:latest
```

Should output JSON-RPC initialization response.

### Step 2: Visit Docker Hub MCP Registry
Go to: **https://hub.docker.com/mcp**

### Step 3: Submit New Server

Click **Submit New Server** or **Add Server**

**Form Fields:**

**Field: Name**
```
toRustCalcMCP
```

**Field: Image**
```
carlomagnoglobal/torustcalcmcp
```
(or with specific tag: `carlomagnoglobal/torustcalcmcp:latest`)

**Field: Description**
```
Exact-rational arbitrary-precision calculator with 351 mathematical functions.
Implements transcendental functions, special functions, complex numbers,
list/string/matrix operations, file I/O, and system integration.
Rust port of the classic calc language.
```

**Field: Documentation URL**
```
https://github.com/carlomagnoglobal/toRustCalcMCP#readme
```

**Field: Source Repository**
```
https://github.com/carlomagnoglobal/toRustCalcMCP
```

**Field: MCP Configuration** (Docker-focused)
```json
{
  "mcpServers": {
    "toRustCalcMCP": {
      "command": "docker",
      "args": ["run", "-i", "--rm", "carlomagnoglobal/torustcalcmcp:latest"]
    }
  }
}
```

**Field: Tags/Categories**
```
calculator, math, utility, docker
```

**Field: Installation Instructions**
```
1. Pull the Docker image:
   docker pull carlomagnoglobal/torustcalcmcp:latest

2. Test it works:
   echo '{"jsonrpc":"2.0","id":1,"method":"initialize","params":{}}' | \
     docker run -i --rm carlomagnoglobal/torustcalcmcp:latest

3. Add to your Claude configuration:
   {
     "mcpServers": {
       "toRustCalcMCP": {
         "command": "docker",
         "args": ["run", "-i", "--rm", "carlomagnoglobal/torustcalcmcp:latest"]
       }
     }
   }

4. Restart Claude and use the calculator!
```

### Step 4: Submit
Click **Submit** or **Create**

---

## After Submission

### Registry Listings
- **mcp.so:** https://mcp.so (search for toRustCalcMCP)
- **Docker Hub MCP:** https://hub.docker.com/mcp (search for toRustCalcMCP)
- **Docker Hub Repo:** https://hub.docker.com/r/carlomagnoglobal/torustcalcmcp

### Keeping Updated
Automatic! Every time you create a release:

```bash
git tag v0.1.1
git push origin v0.1.1
```

GitHub Actions automatically:
1. ✅ Builds binaries for all platforms
2. ✅ Creates GitHub Release
3. ✅ Builds and pushes Docker image to Docker Hub
4. ✅ Builds and pushes Docker image to GitHub Container Registry
5. ✅ Registries stay in sync

---

## Command Reference for Users

### Using Docker Hub Image
```bash
# Pull the image
docker pull carlomagnoglobal/torustcalcmcp

# Run the server
docker run -i --rm carlomagnoglobal/torustcalcmcp

# Run with specific version
docker run -i --rm carlomagnoglobal/torustcalcmcp:0.1.0

# List available versions
curl -s https://hub.docker.com/v2/repositories/carlomagnoglobal/torustcalcmcp/tags | jq '.results[].name'
```

### Using GitHub Container Registry (Alternative)
```bash
# Pull the image
docker pull ghcr.io/carlomagnoglobal/toRustCalcMCP

# Run the server
docker run -i --rm ghcr.io/carlomagnoglobal/toRustCalcMCP
```

### Using Pre-built Binary
```bash
# Download
curl -L https://github.com/carlomagnoglobal/toRustCalcMCP/releases/download/v0.1.0/rcalc-x86_64-apple-darwin.tar.gz \
  | tar xz -C /usr/local/bin --strip-components=1

# Run
toRustCalcMCP --mcp
```

### Using Cargo
```bash
# Clone and build
git clone https://github.com/carlomagnoglobal/toRustCalcMCP.git
cd toRustCalcMCP

# Run
cargo run --release --bin toRustCalcMCP -- --mcp
```

---

## Summary Table

| Aspect | mcp.so | Docker Hub MCP |
|--------|--------|----------------|
| **URL** | https://mcp.so/submit | https://hub.docker.com/mcp |
| **Image** | All three options | Docker images |
| **Best For** | All users | Docker users |
| **Primary Config** | Binary (recommended) | Docker Hub |
| **Alternative** | Cargo, Docker | GitHub Container Registry |
| **Auto-Updates** | Via GitHub Releases | Via Docker Hub tags |
| **Setup Time** | 5 min | 15 min (includes Docker Hub setup) |

---

## Recommended Strategy

1. **First:** Submit to **mcp.so** (5 minutes)
   - Use Option A (Pre-built Binary) for broadest user base
   - Anyone can use it

2. **Then:** Setup Docker Hub and submit (15 minutes)
   - Follow DOCKER_HUB_SETUP.md
   - Caters to Docker-focused users

3. **Both registries** point to same project, just different config options

---

## Verification Checklist

### For mcp.so
- [ ] Visit https://mcp.so/submit
- [ ] Fill in all required fields
- [ ] Choose Option A, B, or C for config
- [ ] Submit
- [ ] Verify listing appears on https://mcp.so

### For Docker Hub MCP
- [ ] Create Docker Hub account
- [ ] Create `torustcalcmcp` repository
- [ ] Setup GitHub Actions (DOCKER_HUB_SETUP.md)
- [ ] Create test release tag (v0.1.0)
- [ ] Verify image on Docker Hub
- [ ] Visit https://hub.docker.com/mcp
- [ ] Submit new server
- [ ] Verify listing appears

---

## Complete Setup Timeline

**Total Time: ~20-30 minutes**

1. **0-5 min:** Submit to mcp.so
2. **5-15 min:** Setup Docker Hub and GitHub Actions
3. **15-20 min:** Create release tag to trigger builds
4. **20-30 min:** Submit to Docker Hub MCP Registry

After that, everything is automatic! Each new release tags updates both registries.

---

## Support

- **mcp.so issues:** https://github.com/modelcontextprotocol/docs/issues
- **Docker Hub:** https://hub.docker.com/r/carlomagnoglobal/torustcalcmcp
- **Project issues:** https://github.com/carlomagnoglobal/toRustCalcMCP/issues

---

**Setup Date:** 2025-06-30  
**Status:** Ready for both registry submissions
