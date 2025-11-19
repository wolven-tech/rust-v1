# MCP Servers Configuration

This file configures Model Context Protocol (MCP) servers for Claude Code CLI (https://claude.com/code).

**Note**: This project uses Claude Code CLI, not Cursor IDE. Configuration is in `.clauderc`, not `.cursorrules`.

## Configured Servers

### Playwright (`@playwright/mcp`)
**Purpose**: Browser automation and end-to-end testing support

**Command**: `bunx @playwright/mcp@latest`

**Capabilities**:
- Run Playwright tests
- Inspect browser automation
- Debug E2E test failures
- Generate test code

**Setup**: No additional configuration required

---

### Sentry (`mcp-server-sentry`)
**Purpose**: Error monitoring and debugging integration

**Command**: `uvx mcp-server-sentry --auth-token ${SENTRY_AUTH_TOKEN}`

**Capabilities**:
- Query error reports
- Analyze stack traces
- Review issue history
- Monitor application health

**Setup**:
1. Get your Sentry auth token from https://sentry.io/settings/account/api/auth-tokens/
2. Add to your environment variables:
   ```bash
   # In apps/app/.env.local
   SENTRY_AUTH_TOKEN=your_sentry_token_here
   ```

**Note**: The MCP server reads `SENTRY_AUTH_TOKEN` from your environment variables automatically.

---

### Trigger.dev (`trigger.dev`)
**Purpose**: Background job management and monitoring

**Command**: `bunx trigger.dev@latest mcp`

**Capabilities**:
- Manage background jobs
- Monitor job execution
- Debug job failures
- Query job history

**Setup**: Requires Trigger.dev project configuration (already set up in apps/app)

---

## Environment Variables

Make sure these are set in your environment:

```bash
# Required for Sentry MCP server
SENTRY_AUTH_TOKEN=your_sentry_token

# Optional but recommended (already in .env.example)
NEXT_PUBLIC_SENTRY_DSN=https://xxx@xxx.ingest.sentry.io/xxx
SENTRY_ORG=your-org
SENTRY_PROJECT=your-project
```

## Usage with Claude Code CLI

These MCP servers are automatically available to Claude Code when configured in `.mcp.json`.

When using Claude Code CLI in this repository, it will:
- Automatically load MCP servers from `.mcp.json`
- Use AI assistant instructions from `.clauderc`
- Have access to enhanced capabilities through MCP servers

Claude Code can use these servers to:
- Run and debug Playwright tests
- Investigate Sentry errors and stack traces
- Monitor and manage Trigger.dev background jobs

**Note**: This is configured for Claude Code CLI, not Cursor IDE or other editors.

## Troubleshooting

### Sentry MCP not working
- Verify `SENTRY_AUTH_TOKEN` is set in your environment
- Check token has correct permissions in Sentry dashboard
- Ensure `uvx` is installed (part of uv package manager)

### Playwright MCP not working
- Ensure Playwright is installed: `bun add -D @playwright/test`
- Verify `bunx` can execute Playwright commands

### Trigger.dev MCP not working
- Check Trigger.dev configuration in `apps/app`
- Verify API credentials are set
- Ensure `bunx` can access `trigger.dev` CLI

## Project AI Configuration

This project uses:
- **`.clauderc`** - Main AI assistant instructions for Claude Code CLI
- **`.mcp.json`** - MCP servers configuration (this file)
- **`.claude/`** - Claude Code specific settings
  - `TDD_CHECKLIST.md` - Test-Driven Development checklist
  - `settings.local.json` - Local user settings (git-ignored)

**Not used**:
- `.cursorrules` - We use Claude Code, not Cursor IDE
- `.aiderignore` - We use Claude Code, not Aider

## References

- [Claude Code Documentation](https://claude.com/code)
- [MCP Documentation](https://modelcontextprotocol.io/)
- [Playwright MCP](https://github.com/playwright/mcp)
- [Sentry MCP](https://github.com/getsentry/mcp-server-sentry)
- [Trigger.dev Documentation](https://trigger.dev/docs)
