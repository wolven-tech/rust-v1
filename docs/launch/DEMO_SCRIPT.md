# Meta Tmux Demo Recording Script

## Recording Instructions

Follow this script when recording the demo with asciinema.

### Preparation (before running `./record-demo.sh`)

1. Close all other terminal windows
2. Clear your terminal: `clear`
3. Make sure you're in the rust-v1 directory
4. Have meta installed and working: `which meta`
5. Set terminal to a good size (120x40 characters recommended for tmux)
6. Use a clean theme with good contrast
7. Make sure tmux is installed: `which tmux`

### Recording Steps

Run the recording script:
```bash
./record-demo.sh
```

When meta starts, follow these steps (SLOWLY - each action should breathe):

#### Step 1: Show meta doctor (5 seconds)
- Run `meta doctor` to show configuration validation
- Let the output display completely
- Shows all tools detected (bacon, cargo, turbo, tmux)
- Validates the setup

#### Step 2: Start Development Servers (3 seconds)
- Run `meta dev`
- Watch the tmux session launch message
- See the navigation guide display
- Let tmux attach and show all panes

#### Step 3: Navigate Between Panes (8 seconds)
- Press `Ctrl+B` then `→` (right arrow) to move between panes
- Navigate through at least 3 different panes
- Pause on each pane to show:
  - Bacon TUI running for Rust projects
  - Turbo dev server for Next.js projects
  - Each with its own interactive interface

#### Step 4: Zoom a Pane (3 seconds)
- Press `Ctrl+B` then `Z` to zoom current pane
- Show the full bacon TUI interface
- Press `Ctrl+B` then `Z` again to unzoom

#### Step 5: Show Pane Numbers (2 seconds)
- Press `Ctrl+B` then `Q` to display pane numbers
- Let the numbers show briefly

#### Step 6: Detach from Session (3 seconds)
- Press `Ctrl+B` then `D` to detach
- Show that you're back at the terminal
- Show the processes are still running

#### Step 7: Reattach to Session (2 seconds)
- Run `tmux attach -t meta-dev`
- Show seamless reconnection to running services

#### Step 8: Stop a Service (3 seconds)
- Navigate to one pane
- Press `Ctrl+C` to stop that service
- Show the exit message

#### Step 9: Exit Cleanly (2 seconds)
- Press `Ctrl+B` then `X` to close a pane
- Type `y` to confirm
- Or navigate to each pane and press `Ctrl+C`

### Total Time: ~30-35 seconds of active demo

## Key Things to Highlight

Visual elements that should be captured:

1. **Doctor Validation** - Quick health check of all tools
2. **Instant Launch** - How fast tmux session starts
3. **Multiple Bacon TUIs** - Each Rust project gets full interactive TUI
4. **Turborepo Integration** - Next.js apps running from workspace root
5. **Navigation Guide** - Helpful keyboard shortcuts displayed
6. **Smooth Navigation** - Easy movement between panes
7. **Zoom Feature** - Focus on one service full screen
8. **Detach/Reattach** - Processes keep running in background
9. **Clean Exit** - Graceful shutdown of services

## After Recording

### Option 1: Convert to GIF with agg (Recommended)

Install agg:
```bash
cargo install --git https://github.com/asciinema/agg
```

Convert to GIF:
```bash
agg meta-demo.cast meta-demo.gif
```

With custom settings:
```bash
agg \
  --font-size 14 \
  --theme monokai \
  --speed 1.2 \
  meta-demo.cast \
  meta-demo.gif
```

### Option 2: Upload to asciinema.org

```bash
asciinema upload meta-demo.cast
```

This gives you a shareable URL and embeddable player.

### Option 3: Convert to SVG

```bash
npm install -g svg-term-cli
cat meta-demo.cast | svg-term --out meta-demo.svg --window
```

## Tips for Best Results

1. **Go Slower Than You Think** - Each action should be visible
2. **Pause Between Actions** - Let the viewer absorb what happened
3. **Show Real Development** - Run actual dev servers so there's real output
4. **Demonstrate Bacon Features** - Show how bacon TUI works in each pane
5. **Keep Terminal Wide** - Tmux works best with wider terminals (120+ columns)
6. **Practice First** - Do a dry run before recording

## What If You Mess Up?

Just run `./record-demo.sh` again. It's quick to re-record.

## File Sizes

Expected file sizes:
- `.cast` file: ~100-300KB (depending on length)
- `.gif` file: 2-8MB (can be optimized)

For social media:
- Twitter/X: Max 15MB for GIF
- LinkedIn: Max 200MB but recommend <10MB
- Instagram: Not ideal, use video format instead

## Alternative: Screen Recording

If asciinema conversion is problematic:

1. Use macOS Screenshot tool (Cmd+Shift+5)
2. Record the terminal window
3. Convert with ffmpeg:
```bash
ffmpeg -i screen-recording.mov \
  -vf "fps=15,scale=1200:-1:flags=lanczos" \
  -c:v gif \
  meta-demo.gif
```

## Quick Reference: Tmux Keyboard Shortcuts

| Key Combo | Action |
|-----------|--------|
| `Ctrl+B` then `→/←/↑/↓` | Navigate between panes |
| `Ctrl+B` then `Z` | Zoom/unzoom current pane |
| `Ctrl+B` then `Q` | Show pane numbers |
| `Ctrl+B` then `D` | Detach from session |
| `Ctrl+B` then `X` | Close current pane |
| `Ctrl+C` | Stop process in current pane |

## Meta Commands Reference

| Command | What It Does |
|---------|--------------|
| `meta doctor` | Validate configuration and tools |
| `meta dev` | Start all dev servers in tmux |
| `meta dev --projects api` | Start specific project only |
| `meta run test` | Run tests across all projects |
| `meta build` | Build all projects |

---

Ready? Run `./record-demo.sh` and follow the steps above!
