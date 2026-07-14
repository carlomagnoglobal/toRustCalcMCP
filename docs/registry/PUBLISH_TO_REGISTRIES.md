# How to Publish toRustCalcMCP to All Registries

Complete step-by-step guide to publish to mcp.so and Docker Hub MCP registries.

---

## 📋 Prerequisites Checklist

Before you start, verify:
- [ ] All tests pass: `cargo test`
- [ ] Project builds: `cargo build --release`
- [ ] Git repo is clean: `git status` shows no changes
- [ ] You're on main branch: `git branch`

---

## ⚙️ Step 1: Setup Docker Hub (5 minutes)

### 1a. Create Docker Hub Account
1. Go to https://hub.docker.com/signup
2. Sign up with email
3. Verify email
4. Note your username

### 1b. Create Docker Hub Repository
1. Log in to https://hub.docker.com
2. Click **Create Repository**
3. Name: `torustcalcmcp` (must be lowercase)
4. Visibility: **Public**
5. Click **Create**

### 1c. Generate Access Token
1. Go to https://hub.docker.com/settings/security
2. Click **New Access Token**
3. Token name: `github-actions`
4. Permissions: "Read & Write"
5. Click **Generate**
6. **COPY THE TOKEN** (you'll only see it once!)

### 1d. Add GitHub Secrets
1. Go to your GitHub repo settings:
   https://github.com/carlomagnoglobal/toRustCalcMCP/settings/secrets/actions

2. Click **New repository secret**
3. Add `DOCKER_HUB_USERNAME`:
   - Name: `DOCKER_HUB_USERNAME`
   - Value: Your Docker Hub username
   - Click **Add secret**

4. Click **New repository secret** again
5. Add `DOCKER_HUB_TOKEN`:
   - Name: `DOCKER_HUB_TOKEN`
   - Value: The token from step 1c
   - Click **Add secret**

✅ **Docker Hub setup complete!**

---

## 📦 Step 2: Create Release (5 minutes)

### 2a. Create Release Tag
```bash
# In your project directory
git tag v0.1.0
git push origin v0.1.0
```

### 2b. Monitor GitHub Actions
1. Go to: https://github.com/carlomagnoglobal/toRustCalcMCP/actions
2. Watch the workflow run
3. Wait for all jobs to complete (about 10 minutes)

### 2c. Verify Artifacts

**Check GitHub Releases:**
- URL: https://github.com/carlomagnoglobal/toRustCalcMCP/releases
- Should see binaries for all platforms

**Check Docker Hub:**
- URL: https://hub.docker.com/r/carlomagnoglobal/torustcalcmcp/tags
- Should see `0.1.0` and `latest` tags

**Check GitHub Container Registry:**
- URL: https://github.com/carlomagnoglobal/toRustCalcMCP/pkgs/container/toRustCalcMCP
- Should see `0.1.0` and `latest` tags

---

## 🎯 Step 3: Submit to mcp.so (5 minutes)

### 3a. Go to Submission Form
Visit: **https://mcp.so/submit**

### 3b. Fill in the Form

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
```

**Field: Categories**
```
Calculator, Math, Utility
```

**Field: Server Config*** - Choose ONE:

**Option A (Recommended - Binary):**
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

**Option B (Cargo):**
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

**Option C (Docker - GitHub Container Registry):**
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

### 3c. Submit
Click the **Submit** button

✅ **Submitted to mcp.so!**

---

## 🐳 Step 4: Submit to Docker Hub MCP Registry (5 minutes)

### 4a. Go to Submission Form
Visit: **https://hub.docker.com/mcp**

### 4b. Click Submit New Server
Look for **Submit New Server** or **Add Server** button

### 4c. Fill in the Form

**Field: Name**
```
toRustCalcMCP
```

**Field: Image**
```
carlomagnoglobal/torustcalcmcp
```
(or with tag: `carlomagnoglobal/torustcalcmcp:0.1.0`)

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

**Field: Tags/Categories**
```
calculator, math, utility
```

**Field: MCP Configuration**
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

**Field: Installation Instructions** (optional)
```
1. Pull the image:
   docker pull carlomagnoglobal/torustcalcmcp:latest

2. Test it:
   echo '{"jsonrpc":"2.0","id":1,"method":"initialize","params":{}}' | \
     docker run -i --rm carlomagnoglobal/torustcalcmcp:latest

3. Add to Claude configuration and restart Claude.
```

### 4d. Submit
Click the **Submit** button

✅ **Submitted to Docker Hub MCP Registry!**

---

## ✅ Verify Both Registries

### Check mcp.so
- URL: https://mcp.so
- Search for "toRustCalcMCP"
- Should appear in results

### Check Docker Hub MCP
- URL: https://hub.docker.com/mcp
- Search for "toRustCalcMCP"
- Should appear in results

---

## 🎉 You're Done!

Your MCP server is now published to:
1. ✅ mcp.so registry
2. ✅ Docker Hub MCP registry
3. ✅ GitHub Releases (automatic)

### For Future Releases

Just repeat Step 2:
```bash
git tag v0.2.0
git push origin v0.2.0
```

Everything else is automatic! GitHub Actions will:
- Build binaries
- Create releases
- Push to Docker Hub
- Push to GitHub Container Registry

Both registries will automatically update.

---

## 🔗 Important Links

### Registries
- mcp.so: https://mcp.so
- Docker Hub MCP: https://hub.docker.com/mcp

### Your Repositories
- GitHub: https://github.com/carlomagnoglobal/toRustCalcMCP
- Docker Hub: https://hub.docker.com/r/carlomagnoglobal/torustcalcmcp
- GitHub Container Registry: https://github.com/carlomagnoglobal/toRustCalcMCP/pkgs/container/toRustCalcMCP

### Actions & Releases
- GitHub Actions: https://github.com/carlomagnoglobal/toRustCalcMCP/actions
- GitHub Releases: https://github.com/carlomagnoglobal/toRustCalcMCP/releases

---

## 📚 Additional Guides

- **DOCKER_HUB_QUICK_START.md** - 5-minute Docker Hub setup
- **DOCKER_HUB_SETUP.md** - Detailed Docker Hub configuration
- **MCP_REGISTRIES_GUIDE.md** - Registry submission details
- **REGISTRY_SUMMARY.md** - Complete registry overview
- **DEPLOYMENT.md** - All deployment options

---

## ⏱️ Time Estimates

| Step | Time |
|------|------|
| Docker Hub setup | 5 min |
| Create release | 5 min |
| GitHub Actions | 10 min |
| Submit to mcp.so | 5 min |
| Submit to Docker Hub | 5 min |
| **Total** | **30 min** |

---

## ❓ Troubleshooting

### Docker Hub Push Failed
- Check GitHub Secrets are set correctly
- Verify token has "Read & Write" permissions
- Check token hasn't expired
- Check GitHub Actions logs for details

### Image Not on Docker Hub
- Wait 2-3 minutes
- Refresh the page
- Check GitHub Actions for build failures

### Can't Find Server on Registries
- Verify submission completed
- Give it 5-10 minutes to appear
- Check exact spelling of server name

### Binary Not on GitHub Releases
- Check GitHub Actions completed successfully
- Verify platform tag is correct
- Wait a few minutes for UI to update

---

## 🎯 Summary

You now have toRustCalcMCP published to three MCP registries with fully automated CI/CD!

**Users can install via:**
- Binary: Download from GitHub Releases
- Docker: `docker pull carlomagnoglobal/torustcalcmcp:latest`
- Cargo: `git clone && cargo run`
- mcp.so registry: Direct integration
- Docker Hub MCP: Docker-focused registry

**You just need to:**
- Create git tags
- GitHub Actions handles the rest!

---

**Last Updated:** 2025-06-30  
**Status:** Ready to publish  
**Estimated Submission Time:** 30 minutes total
