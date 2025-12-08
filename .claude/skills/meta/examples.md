# Meta CLI Examples

## Example 1: Daily Development Start

```bash
# Start the day - launch all dev servers
meta dev

# You're now in tmux with all projects running
# Navigate with Ctrl+B + arrows

# Need to step away? Detach (keeps running)
# Press: Ctrl+B then D

# Come back later and reattach
tmux attach -t meta-dev
```

## Example 2: Check if Process Needs Restart

After making code changes, check if your Rust server picked them up:

```bash
meta status -p api
```

Look for:
- `✓ running latest binary` - All good!
- `⚠️ STALE: binary rebuilt Xm after process started` - Needs restart

If stale and bacon should have restarted:
1. Check bacon is running `run-long` job (not `check`)
2. Check bacon.toml has `on_change_strategy = "kill_then_restart"`

## Example 3: Focus on Single Project

Working only on the API today:

```bash
# Start just the API
meta dev --projects api

# Check its status
meta status -p api -l 30

# Stop when done
meta dev:stop
```

## Example 4: Pre-Commit Checks

Before committing changes:

```bash
# Format all code
meta run fmt

# Run lints
meta run clippy

# Run tests
meta test

# If all pass, you're good to commit
```

## Example 5: Debugging a Crashed Process

Process crashed and won't restart:

```bash
# Check what's happening
meta status

# Look at recent events
meta status -l 50

# Force restart everything
meta dev:stop
meta dev
```

## Example 6: Validate Setup After Config Change

After modifying meta.toml or adding a project:

```bash
# Validate configuration
meta doctor

# If all checks pass, test dev mode
meta dev
```

## Example 7: View Process History

See what's been happening with your dev processes:

```bash
# Recent 20 events
meta status

# More history
meta status -l 100

# Or read the log directly
cat .meta/logs/dev.log
```

## Example 8: Multiple Tmux Sessions

If you need the meta-dev session plus your own tmux:

```bash
# Start meta dev (creates meta-dev session)
meta dev

# Detach from meta-dev
# Press: Ctrl+B then D

# Create your own session for other work
tmux new -s work

# Switch between sessions
tmux switch -t meta-dev
tmux switch -t work

# List all sessions
tmux ls
```

## Example 9: Restart Single Rust Project

If one Rust project is stale but others are fine:

```bash
# Go to that pane in tmux
# Press: Ctrl+B then Q (shows numbers)
# Press the pane number

# In that pane, press Ctrl+C to stop bacon
# Then press Enter to restart (wrapper will re-run)
```

## Example 10: Production Build

Building for deployment:

```bash
# Build all in release mode
meta build --prod

# Or build specific project
meta build --projects api --prod
```
