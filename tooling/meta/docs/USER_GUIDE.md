# Meta User Guide

Complete guide to using Meta for monorepo development orchestration.

## Table of Contents

- [Getting Started](#getting-started)
- [Daily Workflow](#daily-workflow)
- [Tmux Navigation](#tmux-navigation)
- [Recording Demos](#recording-demos)
- [Configuration](#configuration)
- [Troubleshooting](#troubleshooting)

## Getting Started

### Installation

From your monorepo root:

```bash
cd tooling/meta
./install.sh
```

Or install globally:

```bash
cargo install --path tooling/meta
```

### First Time Setup

1. **Validate your setup:**
   ```bash
   meta doctor
   ```

   This checks:
   - ✅ All required tools (bacon, cargo, turbo, tmux)
   - ✅ meta.toml configuration
   - ✅ Project paths
   - ✅ Task definitions

2. **Start development:**
   ```bash
   meta dev
   ```

   This launches a tmux session with:
   - One pane per project
   - Each bacon instance with full interactive TUI
   - Each turbo command from workspace root
   - Helpful navigation guide displayed

## Daily Workflow

### Starting Your Dev Session

```bash
# Check everything is OK
meta doctor

# Start all services
meta dev

# Or start specific projects only
meta dev --projects api web
```

### Navigating Between Services

Meta displays a navigation guide when starting:

```
╭─────────────────────────────────────────────────────────╮
│ 🎮 Tmux Navigation Guide                                │
├─────────────────────────────────────────────────────────┤
│ Navigate Panes:  Ctrl+B then Arrow Keys (← → ↑ ↓)      │
│ Zoom Pane:       Ctrl+B then Z (toggle full screen)    │
│ Show Numbers:    Ctrl+B then Q (then press number)     │
│                                                         │
│ Detach Session:  Ctrl+B then D (keeps running)         │
│ Stop Process:    Ctrl+C (in current pane)              │
│ Close Pane:      Ctrl+B then X (confirm with y)        │
╰─────────────────────────────────────────────────────────╯
```

### Working with Bacon

Each Rust project runs bacon with full interactive TUI:

- **t** - Switch to test mode
- **c** - Switch to clippy mode
- **r** - Switch to run mode
- **w** - Switch to watch mode

Navigate to a bacon pane and use these shortcuts normally!

### Detaching and Reattaching

**Detach** (keeps everything running):
```bash
# Press: Ctrl+B then D
```

**Reattach later:**
```bash
tmux attach -t meta-dev
```

**Kill the session** (stops everything):
```bash
tmux kill-session -t meta-dev
```

### Ending Your Day

**Option 1: Detach (recommended)**
```
Press: Ctrl+B then D
```
Processes keep running. Reattach tomorrow with `tmux attach -t meta-dev`.

**Option 2: Stop everything**
Navigate to each pane and press `Ctrl+C`, or:
```bash
tmux kill-session -t meta-dev
```

## Tmux Navigation

### Essential Shortcuts

All tmux commands start with the **prefix**: `Ctrl+B`

Press `Ctrl+B`, release, then press the next key.

#### Navigation
- `Ctrl+B` then `→` - Move to pane on the right
- `Ctrl+B` then `←` - Move to pane on the left
- `Ctrl+B` then `↑` - Move to pane above
- `Ctrl+B` then `↓` - Move to pane below
- `Ctrl+B` then `O` - Cycle to next pane
- `Ctrl+B` then `Q` - Show pane numbers (then press number to jump)

#### View Management
- `Ctrl+B` then `Z` - Zoom/unzoom current pane (toggle full screen)
- `Ctrl+B` then `{` - Swap with previous pane
- `Ctrl+B` then `}` - Swap with next pane

#### Session Management
- `Ctrl+B` then `D` - Detach from session (keeps running)
- `Ctrl+B` then `X` - Kill current pane (confirms with y/n)

#### Process Control
- `Ctrl+C` - Stop process in current pane (not a tmux command)

### Advanced Tmux

**List all sessions:**
```bash
tmux ls
```

**Attach to specific session:**
```bash
tmux attach -t meta-dev
```

**Create new window in session:**
```
Ctrl+B then C
```

**Switch between windows:**
```
Ctrl+B then N (next)
Ctrl+B then P (previous)
Ctrl+B then [0-9] (window number)
```

## Recording Demos

Want to create a demo video of Meta for documentation or social media?

### Quick Start

```bash
# From the monorepo root
cd docs/launch

# Record the demo (interactive)
./record-demo.sh

# Convert to GIF
./convert-to-gif.sh
```

### What to Record

Follow the script in [docs/launch/DEMO_SCRIPT.md](../../../docs/launch/DEMO_SCRIPT.md):

1. **meta doctor** (5s) - Show validation
2. **meta dev** (3s) - Launch tmux
3. **Navigate panes** (8s) - Show Ctrl+B + arrows
4. **Zoom a pane** (3s) - Show bacon TUI
5. **Pane numbers** (2s) - Show Ctrl+B + Q
6. **Detach** (3s) - Show Ctrl+B + D
7. **Reattach** (2s) - Show tmux attach
8. **Exit** (3s) - Clean shutdown

Total: ~30 seconds (perfect for social media)

### Demo Recording Tips

1. **Terminal size:** 120x40 characters minimum
2. **Go slow:** Each action should be clearly visible
3. **Pause between steps:** Let viewers absorb what happened
4. **Show real output:** Run actual dev servers for authentic logs
5. **Practice first:** Do a dry run before recording

### Converting to Different Formats

**GIF (recommended for social media):**
```bash
./convert-to-gif.sh
```

**Upload to asciinema.org:**
```bash
asciinema upload meta-demo.cast
```

**Convert to SVG:**
```bash
npm install -g svg-term-cli
cat meta-demo.cast | svg-term --out meta-demo.svg --window
```

### File Sizes

- `.cast` file: ~100-300KB
- `.gif` file: 2-8MB (optimized)

Social media limits:
- Twitter/X: Max 15MB
- LinkedIn: Max 200MB (recommend <10MB)

## Configuration

### meta.toml Structure

```toml
version = "1"

[workspace]
name = "My Monorepo"
root = "."

# Tool declarations
[tools.turborepo]
enabled = true
command = "turbo"
for_languages = ["typescript", "javascript"]
for_tasks = ["dev", "build", "lint", "typecheck"]

[tools.bacon]
enabled = true
command = "bacon"
for_languages = ["rust"]
for_tasks = ["dev"]

[tools.cargo]
enabled = true
command = "cargo"
for_languages = ["rust"]
for_tasks = ["build", "test", "clippy"]

# Project definitions
[projects.api]
type = "rust"
path = "apps/api"

[projects.api.tasks]
dev = { tool = "bacon", command = "run-long" }
build = { tool = "cargo", command = "build --release" }
test = { tool = "cargo", command = "test" }

[projects.web]
type = "next"
path = "apps/web"

[projects.web.tasks]
dev = { tool = "turborepo", command = "run dev --filter=@your-org/web" }
build = { tool = "turborepo", command = "run build --filter=@your-org/web" }
```

### Important Notes

**Turborepo commands:**
- Must include `run` keyword
- Use exact package name from package.json in `--filter`
- Meta runs these from workspace root automatically

**Bacon commands:**
- Use job name from bacon.toml (e.g., `run-long`, `check`, `clippy`)
- Meta changes to project directory before running bacon

### Adding New Projects

1. Add project definition:
   ```toml
   [projects.new-service]
   type = "rust"  # or "next"
   path = "apps/new-service"
   ```

2. Add tasks:
   ```toml
   [projects.new-service.tasks]
   dev = { tool = "bacon", command = "check" }
   build = { tool = "cargo", command = "build" }
   ```

3. Validate:
   ```bash
   meta doctor
   ```

4. Test:
   ```bash
   meta dev --projects new-service
   ```

## Troubleshooting

### Tmux Session Won't Start

**Error:** `open terminal failed: not a terminal`

**Solution:** Make sure you're in a real terminal, not an IDE or automation tool.

### Turbo Panes Exit Immediately

**Common causes:**
1. Missing `run` keyword in command
2. Wrong package name in `--filter`
3. Missing dependencies in the app

**Check:**
```bash
meta doctor  # Validates turbo command syntax
```

### Bacon Not Found

**Solution:**
```bash
cargo install bacon
meta doctor  # Verify installation
```

### Can't Navigate Between Panes

**Issue:** Tmux shortcuts not working

**Solution:** Make sure you're using the **prefix** first:
1. Press `Ctrl+B`
2. Release both keys
3. Press the navigation key (arrow, Z, Q, etc.)

### Session Already Exists

**Error:** `duplicate session: meta-dev`

**Solution:**
```bash
# Either attach to existing
tmux attach -t meta-dev

# Or kill and restart
tmux kill-session -t meta-dev
meta dev
```

### Projects Not Detected

**Issue:** `meta init` doesn't find projects

**Solution:** Manually add them to `meta.toml`:
```toml
[projects.your-app]
type = "rust"  # or "next"
path = "path/to/your-app"

[projects.your-app.tasks]
dev = { tool = "bacon", command = "check" }
```

### Command Failed

**Issue:** Task execution failed

**Debug:**
1. Run `meta doctor` to validate setup
2. Try running the command manually from the project directory
3. Check `meta.toml` syntax
4. Verify tool is installed and in PATH

## Getting Help

```bash
meta --help        # General help
meta dev --help    # Command-specific help
meta doctor        # Validate your setup
```

**Documentation:**
- [README](../README.md) - Overview and quick start
- [STANDALONE.md](../STANDALONE.md) - Using Meta in other monorepos
- [Recording Guide](../../../docs/launch/DEMO_SCRIPT.md) - How to record demos

**Issues:**
Report issues at: https://github.com/wolven-tech/rust-v1/issues

---

**Pro Tips:**

1. **Learn tmux basics** - The navigation skills are transferable to any tmux workflow
2. **Use meta doctor often** - Run it after config changes
3. **Start with --projects** - Don't run everything if you only need one service
4. **Detach, don't quit** - Keep your dev session running between breaks
5. **Practice recording** - Do a dry run before recording the real demo
