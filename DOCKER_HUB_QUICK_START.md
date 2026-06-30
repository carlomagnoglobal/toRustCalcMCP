# Docker Hub Quick Setup - 5 Minutes

Fast checklist to get publishing to Docker Hub + Docker MCP Registry.

---

## ✅ Checklist: 5-Minute Setup

### 1. Create Docker Hub Account (2 minutes)
- [ ] Go to https://hub.docker.com/signup
- [ ] Sign up with email
- [ ] Verify email
- [ ] Note your username (you'll need it!)

### 2. Create Repository on Docker Hub (1 minute)
- [ ] Log in to https://hub.docker.com
- [ ] Click **Create Repository**
- [ ] Name: `torustcalcmcp` (must be lowercase)
- [ ] Visibility: **Public**
- [ ] Click **Create**

### 3. Generate Access Token (1 minute)
- [ ] Go to https://hub.docker.com/settings/security
- [ ] Click **New Access Token**
- [ ] Token name: `github-actions`
- [ ] Permissions: "Read & Write"
- [ ] Click **Generate**
- [ ] **COPY THE TOKEN** (shown only once!)

### 4. Add Secrets to GitHub (1 minute)
- [ ] Go to: https://github.com/carlomagnoglobal/toRustCalcMCP/settings/secrets/actions
- [ ] Click **New repository secret**
- [ ] Name: `DOCKER_HUB_USERNAME`
- [ ] Value: Your Docker Hub username
- [ ] Click **Add secret**
- [ ] Click **New repository secret** again
- [ ] Name: `DOCKER_HUB_TOKEN`
- [ ] Value: Paste the token from step 3
- [ ] Click **Add secret**

### ✅ Done! You're ready to publish!

---

## Test It: Create a Release

```bash
# In your project directory
git tag v0.1.0-test
git push origin v0.1.0-test
```

Watch the action at: https://github.com/carlomagnoglobal/toRustCalcMCP/actions

When complete, verify on Docker Hub:
```bash
docker pull carlomagnoglobal/torustcalcmcp:0.1.0-test
docker run -i --rm carlomagnoglobal/torustcalcmcp:0.1.0-test
```

Expected: JSON-RPC initialization response

---

## Clean Up Test Tag (Optional)

```bash
# Delete local tag
git tag -d v0.1.0-test

# Delete remote tag
git push origin --delete v0.1.0-test
```

---

## Now Submit to Registries

### Submit to mcp.so
1. Go to https://mcp.so/submit
2. Use this config:
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
3. Submit!

### Submit to Docker Hub MCP Registry
1. Go to https://hub.docker.com/mcp
2. Click **Submit New Server**
3. Use this config:
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
4. Submit!

---

## For Future Releases

Simply create a tag:
```bash
git tag v0.1.1
git push origin v0.1.1
```

GitHub Actions automatically:
- ✅ Builds binaries
- ✅ Creates GitHub Release
- ✅ Pushes to Docker Hub
- ✅ Pushes to GitHub Container Registry

No manual steps needed!

---

## Troubleshooting

**"Failed to push to Docker Hub"**
- [ ] Check secrets are set correctly (Settings → Secrets)
- [ ] Check token has "Read & Write" permissions
- [ ] Check token hasn't expired
- [ ] Check GitHub Actions log for exact error

**"Image not showing on Docker Hub"**
- [ ] Wait 2-3 minutes
- [ ] Refresh https://hub.docker.com/r/carlomagnoglobal/torustcalcmcp/tags
- [ ] Check GitHub Actions for build errors

**"Can't pull image locally"**
```bash
# Verify it's available
docker pull carlomagnoglobal/torustcalcmcp:latest

# Check Docker Hub web interface
# https://hub.docker.com/r/carlomagnoglobal/torustcalcmcp
```

---

## Resources

- Full Docker Hub setup: DOCKER_HUB_SETUP.md
- MCP Registries guide: MCP_REGISTRIES_GUIDE.md
- GitHub Actions workflow: .github/workflows/release.yml

---

**Time to complete:** ~5 minutes  
**Difficulty:** Very Easy ✅
