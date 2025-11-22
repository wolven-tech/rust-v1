# Launch Documentation

This directory contains everything you need to launch and announce the rust-v1 monorepo and Meta orchestrator on social media.

## Demo Video

Watch the official demo:

[![asciicast](https://asciinema.org/a/N9NaULQ470ih2SV6aymeENmLq.svg)](https://asciinema.org/a/N9NaULQ470ih2SV6aymeENmLq)

Use this video in your social media posts!

## Quick Start

1. **Record the demo:**
   ```bash
   cd docs/launch
   ./record-demo.sh
   ```

2. **Convert to GIF:**
   ```bash
   ./convert-to-gif.sh
   ```

3. **Launch on social media:**
   - Open `SOCIAL_MEDIA_POSTS.md` for ready-to-copy posts
   - Follow `LAUNCH_CHECKLIST.md` for timing and strategy

## Files in This Directory

### Documentation
- **`SOCIAL_MEDIA_POSTS.md`** - Viral-style posts for LinkedIn, X/Twitter, and Instagram
- **`DEMO_SCRIPT.md`** - Step-by-step recording guide with timing for tmux demo
- **`LAUNCH_CHECKLIST.md`** - Complete launch strategy and engagement tactics

### Scripts
- **`record-demo.sh`** - Records meta tmux demo with asciinema
- **`convert-to-gif.sh`** - Converts recording to optimized GIF

## Generated Files (Not Committed)

After running the scripts, you'll have:
- `meta-demo.cast` - asciinema recording (shareable)
- `meta-demo.gif` - Optimized GIF for social media

## Platform Guidelines

### LinkedIn
- Best time: Tuesday-Thursday, 8-10 AM or 12-1 PM
- Use 3-5 hashtags
- Pin to profile for first 24 hours
- Respond to comments within 1 hour

### X/Twitter
- Best time: Tuesday-Thursday, 9 AM or 5-6 PM
- Thread format recommended
- Pin first tweet
- Engage immediately

### Distribution Strategy
See `LAUNCH_CHECKLIST.md` for:
- Complete platform distribution plan
- First 24-hour engagement strategy
- Week 1 follow-up tactics
- Crisis management procedures

## Content Principles

All posts follow viral LinkedIn content principles from Jan Tegze:
- Strong personal hook
- Clear problem → solution narrative
- Concrete metrics and value props
- Transparency about early access
- Call-to-action questions

## What Makes Meta Special

Meta v0.2.1 is a **Rust-powered monorepo orchestrator** with these unique features:

### 🥓 Multiple Bacon Instances
Each Rust project gets its own bacon instance running with full interactive TUI in a dedicated tmux pane. No output capture, no hacks - just native bacon TUI in all its glory.

### ⚡ Turborepo Integration
Proper workspace-aware task execution. Turbo commands run from workspace root with correct `--filter` syntax. Meta understands monorepo architecture.

### 🖥️ Tmux Orchestration
Professional terminal multiplexing. Navigate between panes with keyboard shortcuts, detach/reattach sessions, zoom individual panes. All the power of tmux with intelligent setup.

### 🎯 Zero Configuration
Auto-detects Rust and Next.js projects. Generates sensible defaults. Just run `meta init` and you're ready to go.

### ✅ Built-in Validation
`meta doctor` validates your entire setup - checks tool availability, project paths, configuration syntax, everything. Know what's wrong before you start.

## Demo Recording Strategy

The demo should showcase:

1. **Doctor Validation** (5s) - Show health check of all tools
2. **Instant Launch** (3s) - `meta dev` starts tmux session immediately
3. **Multiple Panes** (8s) - Navigate between bacon and turbo instances
4. **Zoom Feature** (3s) - Full-screen focus on one service
5. **Pane Numbers** (2s) - Quick navigation with numbers
6. **Detach/Reattach** (5s) - Processes keep running in background
7. **Clean Exit** (3s) - Graceful shutdown

**Total time:** 30-35 seconds (perfect for social media attention span)

## Key Messaging

Focus on these pain points:

**Before Meta:**
- 10+ terminal tabs for different services
- Mixed logs, can't tell what failed
- Manual turborepo --filter commands
- Bacon instances fighting for terminal
- No way to detach and keep things running

**After Meta:**
- One command: `meta dev`
- Each service in its own pane
- Full bacon TUI for each Rust project
- Turbo running from workspace root
- Detach/reattach anytime
- Helpful navigation guide on startup

## Target Audiences

1. **Rust Developers** - "Finally, proper bacon orchestration"
2. **Full-Stack Teams** - "Rust + Next.js development made simple"
3. **DevOps Engineers** - "Reproducible dev environments"
4. **Productivity Enthusiasts** - "From chaos to organized tmux panes"

See `SOCIAL_MEDIA_POSTS.md` for audience-specific messaging.

## Support

For questions or improvements to the launch materials, open an issue or PR.

---

Built with the goal of authentic, transparent community building.
