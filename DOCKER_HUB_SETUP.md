# Docker Hub Setup & MCP Registry Publishing

This guide explains how to publish toRustCalcMCP to both **Docker Hub** and the **Docker MCP Hub Registry**.

---

## Step 1: Create Docker Hub Account & Repository

### 1a. Sign Up for Docker Hub
1. Go to https://hub.docker.com
2. Click **Sign Up**
3. Create account with username/email (e.g., `carlomagnoglobal`)
4. Verify email

### 1b. Create Repository
1. Log in to https://hub.docker.com
2. Click **Create Repository**
3. Fill in:
   - **Repository Name:** `torustcalcmcp`
   - **Description:** `Rust port of calc - exact-rational calculator as MCP server`
   - **Visibility:** Public
   - **Short Description:** `Exact-rational arbitrary-precision calculator with 351 math functions`
4. Click **Create**

### 1c. Enable Build (Optional - CI/CD already handles this)
Skip this if using GitHub Actions (which we are)

---

## Step 2: Generate Docker Hub Access Token

### 2a. Create Access Token
1. Go to https://hub.docker.com/settings/security
2. Click **New Access Token**
3. Fill in:
   - **Token name:** `github-actions`
   - **Access permissions:** Select "Read & Write"
4. Click **Generate**
5. **Copy the token** (you'll only see it once!)

### 2b. Store Token Securely
**Do NOT commit this token to GitHub!**

---

## Step 3: Add Secrets to GitHub

### 3a. Add to GitHub Repository Secrets
1. Go to your repository: https://github.com/carlomagnoglobal/toRustCalcMCP
2. Click **Settings** → **Secrets and variables** → **Actions**
3. Click **New repository secret**
4. Add two secrets:

**Secret 1:**
- **Name:** `DOCKER_HUB_USERNAME`
- **Value:** Your Docker Hub username (e.g., `carlomagnoglobal`)
- Click **Add secret**

**Secret 2:**
- **Name:** `DOCKER_HUB_TOKEN`
- **Value:** The access token you created in Step 2a
- Click **Add secret**

✅ **Done!** GitHub Actions can now push to Docker Hub

---

## Step 4: Test the Setup

### 4a. Create a Test Release
```bash
# Add and commit changes
git add -A
git commit -m "Add Docker Hub publishing"

# Create a test tag
git tag v0.1.0-test
git push origin v0.1.0-test
```

### 4b. Monitor the Build
1. Go to GitHub Actions: https://github.com/carlomagnoglobal/toRustCalcMCP/actions
2. Click the workflow run for your tag
3. Watch the `build-docker` job complete
4. Check Docker Hub: https://hub.docker.com/r/carlomagnoglobal/torustcalcmcp/tags

### 4c. Verify Images Pushed
```bash
# List tags on Docker Hub
curl -s https://hub.docker.com/v2/repositories/carlomagnoglobal/torustcalcmcp/tags | jq '.results[].name'

# Or pull the image
docker pull carlomagnoglobal/torustcalcmcp:0.1.0-test
docker run -i --rm carlomagnoglobal/torustcalcmcp:0.1.0-test <<'EOF'
{"jsonrpc":"2.0","id":1,"method":"initialize","params":{}}
EOF
```

---

## Step 5: Publish to Docker MCP Hub Registry

### 5a. Go to Docker Hub MCP Registry
Visit: https://hub.docker.com/mcp

### 5b. Submit Your Server
Click **Submit New Server** or **Add Server**

Fill in the form with:

**Basic Information:**
- **Name:** `toRustCalcMCP`
- **Description:** Rust port of calc - exact-rational calculator with 351 mathematical functions. Implements transcendental functions, special functions (Bessel, Gamma, Zeta), complex numbers, list/string operations, file I/O, matrix operations, and full system integration.
- **Repository URL:** `https://github.com/carlomagnoglobal/toRustCalcMCP`
- **Docker Image:** `carlomagnoglobal/torustcalcmcp` (or `carlomagnoglobal/torustcalcmcp:latest`)

**Server Configuration:**
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

**Tags/Categories:**
- `calculator`
- `math`
- `utility`
- `docker`

**Installation Instructions:**
```
1. Pull the image:
   docker pull carlomagnoglobal/torustcalcmcp:latest

2. Run the server:
   docker run -i --rm carlomagnoglobal/torustcalcmcp:latest

3. Configure in Claude:
   Add the configuration above to your Claude MCP settings
```

**Documentation Link:**
`https://github.com/carlomagnoglobal/toRustCalcMCP#readme`

Click **Submit** or **Create**

---

## Configuration Summary

### GitHub Actions (Automatic)
- **When:** On each git tag (e.g., `git tag v0.1.0`)
- **Registries:** Pushes to both Docker Hub AND GitHub Container Registry
- **Tags:** version, latest, sha, branch

### Docker Hub MCP Registry
- **Registry:** https://hub.docker.com/mcp
- **Image URL:** `carlomagnoglobal/torustcalcmcp`
- **Latest:** `carlomagnoglobal/torustcalcmcp:latest`

### Both Registries

**GitHub Container Registry:**
```
ghcr.io/carlomagnoglobal/toRustCalcMCP:latest
ghcr.io/carlomagnoglobal/toRustCalcMCP:0.1.0
```

**Docker Hub:**
```
docker.io/carlomagnoglobal/torustcalcmcp:latest
docker.io/carlomagnoglobal/torustcalcmcp:0.1.0
docker pull carlomagnoglobal/torustcalcmcp  # Shorthand
```

---

## CLI Commands for Users

### Using Docker Hub
```bash
# Pull
docker pull carlomagnoglobal/torustcalcmcp:latest

# Run
docker run -i --rm carlomagnoglobal/torustcalcmcp:latest

# Test
echo '{"jsonrpc":"2.0","id":1,"method":"initialize","params":{}}' | \
  docker run -i --rm carlomagnoglobal/torustcalcmcp:latest
```

### Using GitHub Container Registry (Alternative)
```bash
# Pull
docker pull ghcr.io/carlomagnoglobal/toRustCalcMCP:latest

# Run
docker run -i --rm ghcr.io/carlomagnoglobal/toRustCalcMCP:latest
```

---

## MCP Configuration for Users

**Option 1: Docker Hub (Recommended for Docker Hub Registry)**
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

**Option 2: GitHub Container Registry (Alternative)**
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

## Troubleshooting

### Docker Hub Push Failed
- ✅ Check GitHub Actions logs for error
- ✅ Verify `DOCKER_HUB_USERNAME` and `DOCKER_HUB_TOKEN` are set
- ✅ Verify token has "Read & Write" permissions
- ✅ Check token hasn't expired
- ✅ Verify repository exists on Docker Hub

### Image Not Showing on Docker Hub
- ✅ Check Docker Hub dashboard: https://hub.docker.com/r/carlomagnoglobal/torustcalcmcp/tags
- ✅ Wait a few minutes (sometimes takes time to appear)
- ✅ Check GitHub Actions for build failures
- ✅ Verify tag format is correct (e.g., v0.1.0)

### Token Issues
- ✅ Go to https://hub.docker.com/settings/security
- ✅ Revoke old token
- ✅ Create new token
- ✅ Update GitHub secret

### MCP Registry Submission Issues
- ✅ Use exact image URL: `carlomagnoglobal/torustcalcmcp`
- ✅ Ensure image is pushed and available
- ✅ Test locally first: `docker pull carlomagnoglobal/torustcalcmcp:latest`

---

## Verification Checklist

Before considering setup complete:

- [ ] Docker Hub account created
- [ ] Repository created on Docker Hub
- [ ] Access token generated (from hub.docker.com/settings/security)
- [ ] `DOCKER_HUB_USERNAME` secret added to GitHub
- [ ] `DOCKER_HUB_TOKEN` secret added to GitHub
- [ ] Workflow file updated (.github/workflows/release.yml)
- [ ] Test release created and built successfully
- [ ] Image appears on Docker Hub tags page
- [ ] Image can be pulled: `docker pull carlomagnoglobal/torustcalcmcp:latest`
- [ ] Image runs successfully with test
- [ ] Submitted to https://hub.docker.com/mcp (Docker MCP Hub Registry)

---

## Release Workflow

Once setup, the process is simple:

```bash
# 1. Make sure code is ready
cargo test
cargo build --release

# 2. Create tag
git tag v0.1.0

# 3. Push to trigger CI/CD
git push origin v0.1.0

# 4. Watch GitHub Actions
# → Builds binaries
# → Creates GitHub Release
# → Builds Docker image
# → Pushes to Docker Hub
# → Pushes to GitHub Container Registry

# 5. Verify on Docker Hub
# https://hub.docker.com/r/carlomagnoglobal/torustcalcmcp/tags

# 6. Users can now pull and use
docker pull carlomagnoglobal/torustcalcmcp:0.1.0
```

---

## Summary

✅ **Automatic publishing to both registries**
✅ **GitHub Actions handles all building**
✅ **No manual Docker commands needed**
✅ **Users can pull from either registry**
✅ **Registered on Docker MCP Hub**

Everything is automated after the initial setup! 🎉

---

## Support

- **Docker Hub:** https://hub.docker.com/r/carlomagnoglobal/torustcalcmcp
- **GitHub Container Registry:** https://github.com/carlomagnoglobal/toRustCalcMCP/pkgs/container/toRustCalcMCP
- **GitHub Issues:** https://github.com/carlomagnoglobal/toRustCalcMCP/issues
- **Docker Hub MCP Registry:** https://hub.docker.com/mcp

---

**Setup Date:** 2025-06-30  
**Status:** Ready for Docker Hub publishing
