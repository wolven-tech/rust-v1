# Meta TUI Demo Recording Script

## Recording Instructions

Follow this script when recording the demo with asciinema.

### Preparation (before running `./record-demo.sh`)

1. Close all other terminal windows
2. Clear your terminal: `clear`
3. Make sure you're in the rust-v1 directory
4. Have meta installed and working: `which meta`
5. Set terminal to a good size (100x30 characters recommended)
6. Use a clean theme with good contrast

### Recording Steps

Run the recording script:
```bash
./record-demo.sh
```

When meta TUI starts, follow these steps (SLOWLY - each action should breathe):

#### Step 1: Let TUI Load (3 seconds)
- Don't touch anything
- Let all services initialize
- Watch the status indicators

#### Step 2: Navigate Services (5 seconds)
- Press `↓` (or `j`) to move down through services
- Press `↑` (or `k`) to move up through services
- Pause on each service for 1 second
- Show at least 3 different services

#### Step 3: Filter Logs (3 seconds)
- Select the API service (or one with active logs)
- Press `Enter` to filter logs for that service
- Let filtered logs stream for 2-3 seconds
- Show how it isolates just one service's output

#### Step 4: Show All Logs (2 seconds)
- Press `a` to remove filter and show all logs
- Let mixed logs stream briefly
- Show the color coding (red errors, white info)

#### Step 5: Clear Buffer (2 seconds)
- Press `c` to clear the log buffer
- Watch the screen clear
- New logs start streaming immediately

#### Step 6: Final Navigation (2 seconds)
- Navigate through services one more time
- Show the responsiveness

#### Step 7: Quit Gracefully (2 seconds)
- Press `q` to quit
- Show clean exit

### Total Time: ~20 seconds of active demo

## Key Things to Highlight

Visual elements that should be captured:

1. **Startup Speed** - How fast the TUI loads
2. **Color Coding** - Red for errors, white for info/debug
3. **Service List** - Multiple services shown with status
4. **Smooth Navigation** - Arrow keys/vim keys work instantly
5. **Log Filtering** - One keypress to focus on a service
6. **Real-time Updates** - Logs streaming live
7. **Clean UI** - Well-organized, professional look

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
  --font-size 16 \
  --theme monokai \
  --speed 1.5 \
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
3. **Show Real Logs** - Run actual dev servers so there's real output
4. **Trigger an Error** - If possible, show a red error message
5. **Keep It Short** - 20-30 seconds max for social media

## What If You Mess Up?

Just run `./record-demo.sh` again. It's quick to re-record.

## File Sizes

Expected file sizes:
- `.cast` file: ~50-200KB (depending on length)
- `.gif` file: 1-5MB (can be optimized)

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
  -vf "fps=15,scale=800:-1:flags=lanczos" \
  -c:v gif \
  meta-demo.gif
```

## Quick Reference: TUI Keyboard Shortcuts

| Key | Action |
|-----|--------|
| `↑/k` | Navigate up |
| `↓/j` | Navigate down |
| `Enter` | Filter logs for selected service |
| `a` | Show all logs (remove filter) |
| `c` | Clear log buffer |
| `q` | Quit |

---

Ready? Run `./record-demo.sh` and follow the steps above!
