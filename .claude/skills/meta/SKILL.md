---
name: meta
description: Manage polyglot monorepo dev servers, builds, and tests via meta CLI
---

# Meta CLI Skill

## Quick Command Selection

| Goal | Command |
|------|---------|
| Check what's running | `meta status` |
| Start dev servers | `meta dev` |
| Stop dev servers | `meta dev:stop` |
| View project logs | `meta logs <project>` |
| Stream logs live | `meta logs <project> --follow` |
| List all sessions | `meta sessions` |
| Validate setup | `meta doctor` |
| Run task (fmt/clippy/test) | `meta run <task>` |

## Decision Tree

```
I need to...
├─ Check process status → meta status
├─ Start development
│   ├─ All projects → meta dev
│   └─ Specific project → meta dev --projects <name>
├─ Stop development → meta dev:stop
├─ View logs
│   ├─ Recent output → meta logs <project>
│   ├─ Live stream → meta logs <project> --follow
│   └─ List available → meta logs
├─ Debug issues
│   ├─ Process not running? → meta status, then meta logs <project>
│   ├─ Stale binary? → meta status shows STALE
│   └─ Config problem? → meta doctor
└─ Run quality checks → meta run fmt|clippy|test
```

## Output Parsing

### meta status
```
=== META DEV STATUS ===
Session: meta-<workspace>
Log file: .meta/logs/dev.log

## Running Processes
PROJECT         PID        STARTED                      UPTIME
<name>          <pid>      <datetime>                   <duration>
<name>          -          not running                  -

## Project Logs
  <name> → .meta/logs/<name>.log (<size>)
```

**Extract:**
- Process running: PID is numeric (not "-")
- Process stale: Line contains "STALE"
- Uptime: Last column when running

### meta logs
Raw stdout/stderr from project. Parse for:
- Error patterns: `error:`, `Error:`, `panic`, `FAILED`
- Startup: `Listening on`, `Server started`, `Ready`

## Error Handling

| Exit Code | Meaning | Recovery |
|-----------|---------|----------|
| 0 | Success | - |
| 1 | Config error | Run `meta doctor` |
| 101 | Runtime error | Check `meta status`, `meta logs` |

## Common Workflows

### Start fresh development
```bash
meta doctor          # Validate config
meta dev             # Start all servers
# Ctrl+B D to detach
```

### Debug crashed process
```bash
meta status          # Check what's running
meta logs api -l 100 # View recent logs
meta dev:stop        # Stop all
meta dev             # Restart
```

### Check for stale processes
```bash
meta status
# Look for: STALE: binary rebuilt Xm after process started
# If found: meta dev:stop && meta dev
```

### Manage multiple workspaces
```bash
meta sessions        # List all active meta sessions
# Output shows which workspace each session belongs to
```

## Files

| File | Purpose |
|------|---------|
| `meta.toml` | Project configuration |
| `.meta/logs/dev.log` | Lifecycle events |
| `.meta/logs/<project>.log` | Project stdout/stderr |
