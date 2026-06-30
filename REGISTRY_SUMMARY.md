# toRustCalcMCP - Complete Registry Publishing Guide

Publish to **3 MCP registries** with pre-built binaries, Docker, and source options.

---

## 🎯 The Three Registries

| Registry | URL | Focus | Primary Docker |
|----------|-----|-------|-----------------|
| **1. mcp.so** | https://mcp.so | General MCP registry | Binary/Cargo/ghcr.io |
| **2. Docker Hub MCP** | https://hub.docker.com/mcp | Docker-focused | Docker Hub |
| **3. GitHub Releases** | https://github.com/.../releases | Binary distribution | Direct download |

---

## 📋 Registry Configurations

### Registry 1: mcp.so

**Submit to:** https://mcp.so/submit

**Recommended Configuration (Binary):**
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

**Alternative 1 (Cargo):**
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

**Alternative 2 (GitHub Container Registry):**
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

### Registry 2: Docker Hub MCP

**Submit to:** https://hub.docker.com/mcp

**Configuration (Docker Hub):**
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

**Image Details:**
- **Registry:** Docker Hub
- **Full URL:** `docker.io/carlomagnoglobal/torustcalcmcp`
- **Short URL:** `carlomagnoglobal/torustcalcmcp`
- **Pull Command:** `docker pull carlomagnoglobal/torustcalcmcp:latest`

---

### Registry 3: GitHub Releases

**Access at:** https://github.com/carlomagnoglobal/toRustCalcMCP/releases

**Binaries Available for:**
- ✅ macOS Intel (x86_64)
- ✅ macOS Apple Silicon (aarch64)
- ✅ Linux x86_64
- ✅ Windows x86_64

**Installation Example:**
```bash
# macOS
curl -L https://github.com/carlomagnoglobal/toRustCalcMCP/releases/download/v0.1.0/rcalc-x86_64-apple-darwin.tar.gz \
  | tar xz -C /usr/local/bin --strip-components=1

# Linux
curl -L https://github.com/carlomagnoglobal/toRustCalcMCP/releases/download/v0.1.0/rcalc-linux-x86_64.tar.gz \
  | tar xz -C /usr/local/bin --strip-components=1
```

---

## 🚀 User Cheat Sheet

### Option 1: Pre-built Binary (Fastest)
```bash
# Download from: https://github.com/carlomagnoglobal/toRustCalcMCP/releases
# Install to /usr/local/bin/toRustCalcMCP
# Run: toRustCalcMCP --mcp
# Setup time: 2 minutes
```

### Option 2: Docker Hub
```bash
docker pull carlomagnoglobal/torustcalcmcp:latest
docker run -i --rm carlomagnoglobal/torustcalcmcp:latest
# Setup time: 5 minutes
```

### Option 3: GitHub Container Registry
```bash
docker pull ghcr.io/carlomagnoglobal/toRustCalcMCP:latest
docker run -i --rm ghcr.io/carlomagnoglobal/toRustCalcMCP:latest
# Setup time: 5 minutes
```

### Option 4: Cargo (Source)
```bash
git clone https://github.com/carlomagnoglobal/toRustCalcMCP.git
cd toRustCalcMCP
cargo run --release -- --mcp
# Setup time: 10 minutes
```

---

## 📖 Submission Steps

### Step 1: Prepare (One-time)
- [ ] Follow DOCKER_HUB_QUICK_START.md for Docker Hub setup
- [ ] Create first release tag: `git tag v0.1.0 && git push origin v0.1.0`
- [ ] Wait for GitHub Actions to complete
- [ ] Verify images appear on:
  - [ ] GitHub Releases: https://github.com/carlomagnoglobal/toRustCalcMCP/releases
  - [ ] Docker Hub: https://hub.docker.com/r/carlomagnoglobal/torustcalcmcp/tags
  - [ ] GitHub Container Registry: https://github.com/carlomagnoglobal/toRustCalcMCP/pkgs/container/toRustCalcMCP

### Step 2: Submit to mcp.so
- [ ] Go to: https://mcp.so/submit
- [ ] Fill name: `toRustCalcMCP`
- [ ] Fill URL: `https://github.com/carlomagnoglobal/toRustCalcMCP`
- [ ] Choose config (recommend: Binary)
- [ ] Submit

### Step 3: Submit to Docker Hub MCP
- [ ] Go to: https://hub.docker.com/mcp
- [ ] Click "Submit New Server"
- [ ] Image: `carlomagnoglobal/torustcalcmcp`
- [ ] Use Docker Hub config
- [ ] Submit

### Step 4: Done!
- [ ] Monitor https://mcp.so (search for toRustCalcMCP)
- [ ] Monitor https://hub.docker.com/mcp (search for toRustCalcMCP)

---

## 🔄 Automated Publishing Workflow

### On Each Release Tag

When you run:
```bash
git tag v0.1.1
git push origin v0.1.1
```

GitHub Actions **automatically**:

1. **Build Binaries** (all platforms)
   - macOS Intel & ARM
   - Linux (x86_64 & aarch64)
   - Windows

2. **Create GitHub Release**
   - Upload all binaries
   - Available at: https://github.com/carlomagnoglobal/toRustCalcMCP/releases

3. **Build Docker Image**
   - Push to Docker Hub
   - Available at: https://hub.docker.com/r/carlomagnoglobal/torustcalcmcp

4. **Push to GitHub Container Registry**
   - Available at: https://ghcr.io/carlomagnoglobal/toRustCalcMCP

**Everything happens automatically - no manual steps!**

---

## 📊 Registry Comparison

| Feature | mcp.so | Docker Hub | GitHub Releases |
|---------|--------|-----------|-----------------|
| **Setup** | 5 min | 15 min | Automatic |
| **Visibility** | Curated | Docker users | GitHub users |
| **Binaries** | ✅ | ❌ | ✅ |
| **Docker** | ✅ (ghcr) | ✅ | ❌ |
| **Source** | ✅ | ❌ | ❌ |
| **Auto-Updates** | ✅ | ✅ | ✅ |
| **User Base** | All | Docker | Developers |

---

## 🎯 Recommended Setup Timeline

**Total Time: ~30 minutes**

1. **Setup Docker Hub** (5 min)
   - Follow DOCKER_HUB_QUICK_START.md
   - Add GitHub secrets

2. **Create Release** (5 min)
   - `git tag v0.1.0 && git push origin v0.1.0`

3. **Wait for Actions** (10 min)
   - Watch GitHub Actions complete
   - Verify artifacts appear

4. **Submit Registries** (10 min)
   - Submit to mcp.so (5 min)
   - Submit to Docker Hub MCP (5 min)

**Then:** Automated forever! Just tag releases.

---

## 📦 Distribution by Registry

### mcp.so Users
- Can use: **Binary** (recommended) + Cargo + Docker
- Most flexible option

### Docker Hub MCP Users
- Can use: **Docker Hub image** (carlomagnoglobal/torustcalcmcp)
- Docker-focused

### GitHub Users
- Can use: **Binary downloads** from releases
- No Docker needed

---

## 🔗 Links Summary

| Purpose | URL |
|---------|-----|
| **MCP Registry 1** | https://mcp.so/submit |
| **MCP Registry 2** | https://hub.docker.com/mcp |
| **Docker Hub Repo** | https://hub.docker.com/r/carlomagnoglobal/torustcalcmcp |
| **GitHub Releases** | https://github.com/carlomagnoglobal/toRustCalcMCP/releases |
| **GitHub Actions** | https://github.com/carlomagnoglobal/toRustCalcMCP/actions |
| **GitHub Container Registry** | https://github.com/carlomagnoglobal/toRustCalcMCP/pkgs/container/toRustCalcMCP |
| **Project Home** | https://github.com/carlomagnoglobal/toRustCalcMCP |

---

## 📚 Documentation Files

- **DOCKER_HUB_QUICK_START.md** - 5-minute setup checklist
- **DOCKER_HUB_SETUP.md** - Detailed Docker Hub guide
- **MCP_REGISTRIES_GUIDE.md** - Both registry submissions
- **MCP_SO_SUBMISSION.md** - mcp.so form data
- **DEPLOYMENT.md** - All deployment options
- **README.md** - Full project documentation

---

## ✅ Final Checklist

Before publishing:

**Code:**
- [ ] All tests pass: `cargo test`
- [ ] Builds cleanly: `cargo build --release`
- [ ] No warnings in logs

**Docker Hub Setup:**
- [ ] Account created
- [ ] Repository created
- [ ] Access token generated
- [ ] GitHub secrets added

**Automation:**
- [ ] GitHub Actions workflow updated
- [ ] Release job can push to Docker Hub
- [ ] GitHub Container Registry configured

**Release:**
- [ ] Create tag: `git tag v0.1.0`
- [ ] Push tag: `git push origin v0.1.0`
- [ ] Watch Actions complete
- [ ] Verify artifacts:
  - [ ] GitHub Releases has binaries
  - [ ] Docker Hub has image
  - [ ] GitHub Container Registry has image

**Registries:**
- [ ] Submitted to mcp.so
- [ ] Submitted to Docker Hub MCP
- [ ] Both listings active

---

## 🎉 Success Indicators

You'll know everything is working when:

✅ `docker pull carlomagnoglobal/torustcalcmcp:latest` works  
✅ `docker run -i --rm ... --mcp` outputs JSON-RPC response  
✅ Binaries appear on GitHub Releases  
✅ Both registry listings are live  
✅ `mcp.so` search finds toRustCalcMCP  
✅ `hub.docker.com/mcp` search finds toRustCalcMCP  

---

**Setup Date:** 2025-06-30  
**Status:** ✅ Ready for Multi-Registry Publishing  
**Total Registries:** 3 (mcp.so, Docker Hub MCP, GitHub Releases)
