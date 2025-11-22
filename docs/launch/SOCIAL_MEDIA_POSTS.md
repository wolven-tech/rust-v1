# Social Media Update Posts - Meta v0.2.1

## LinkedIn Version

---

Remember Meta, the monorepo orchestrator I shared a while back?

I just shipped v0.2.1 with a complete architectural pivot.

**The problem with the old approach:**
- I was trying to capture bacon's output
- Lost the interactive TUI that makes bacon amazing
- Fighting against how the tool was designed to work

**The realization:**
Stop fighting. Start orchestrating.

**What's new in v0.2.1:**

🥓 **Multiple Bacon TUIs** - Each Rust project gets its own bacon instance with FULL interactive TUI (no more output capture)

🖥️ **Tmux Orchestration** - Each process runs in its own pane with native terminal access

⚡ **Turborepo Integration** - Proper workspace-aware execution from the root

✅ **Built-in Validation** - `meta doctor` checks your entire setup before you start

The workflow is dead simple:

```bash
meta doctor  # Validate everything
meta dev     # Launch tmux with all services
```

Then use tmux shortcuts to navigate:
- `Ctrl+B` + arrows to switch between panes
- `Ctrl+B` + Z to zoom into one service
- `Ctrl+B` + D to detach (keeps running)

Each bacon instance runs with its full TUI. You can interact with it just like running bacon directly. No hacks, no output parsing, no fighting the ecosystem.

**Why this matters:**

Bacon is designed to BE the orchestrator for Rust. But in a monorepo with Next.js apps too? You need something that coordinates multiple bacon instances AND turborepo tasks.

That's Meta.

⚠️ **Early access warning:** Still in active development (v0.2.1). Core is solid, but we're adding features. Use it, break it, tell me what's missing.

**Demo:** https://asciinema.org/a/N9NaULQ470ih2SV6aymeENmLq

GitHub: https://github.com/wolven-tech/rust-v1

Quick start:
```bash
bunx degit wolven-tech/rust-v1 my-project
cd my-project && bun install
cd tooling/meta && ./install.sh && cd ../..
meta dev
```

For those who tried the old TUI version - this is way better. Each tool gets its own space with full interactivity.

What do you think of this approach?

---

## X (Twitter) Version - Thread

---

**Tweet 1:**
Meta v0.2.1 just dropped with a complete rewrite.

Old approach: Capture bacon output in a TUI
New approach: Orchestrate multiple bacon instances in tmux

Why? Because fighting bacon is stupid. It's already a TUI. Let it be one.

Thread ↓

---

**Tweet 2:**
What changed:

🥓 Each Rust project = its own bacon instance with FULL TUI
🖥️ Tmux gives each process its own pane
⚡ Turborepo runs from workspace root (proper --filter)
✅ meta doctor validates your setup

One command: `meta dev`

All processes running natively.

---

**Tweet 3:**
The old version tried to be smart.
Capture output. Parse logs. Build a dashboard.

The new version is smarter.
Use tmux. Give each process a pane. Let bacon be bacon.

Sometimes the best solution is the simple one.

---

**Tweet 4:**
For Rust devs:

Each bacon instance runs exactly like you'd run it manually.
Full keyboard interaction.
Switch jobs (t for test, c for clippy).
All the TUI goodness.

But now you can run multiple projects + Next.js apps at once.

---

**Tweet 5:**
v0.2.1 is live: https://github.com/wolven-tech/rust-v1

Watch the demo: https://asciinema.org/a/N9NaULQ470ih2SV6aymeENmLq

⚠️ Still early. Core is solid but features are coming.

If you tried the old TUI version - this is the real deal.

What feature should I add next?

---

## X (Twitter) Version - Single Tweet

---

Meta v0.2.1: Complete rewrite

Old: Custom TUI capturing bacon output
New: Tmux orchestration with native bacon TUIs

Each Rust project gets full interactive bacon.
Turborepo runs from workspace root.
One command: `meta dev`

Better. Simpler. Faster.

Watch demo: https://asciinema.org/a/N9NaULQ470ih2SV6aymeENmLq
GitHub: https://github.com/wolven-tech/rust-v1

---

## Creating the Demo GIF

### Quick Start (2 steps)

1. **Record the demo:**
   ```bash
   cd docs/launch
   ./record-demo.sh
   ```
   Follow the on-screen instructions and the detailed steps in `DEMO_SCRIPT.md`

2. **Convert to GIF:**
   ```bash
   ./convert-to-gif.sh
   ```
   This will install `agg` if needed and create `meta-demo.gif`

### What Gets Recorded (30-35 seconds)

1. meta doctor - Configuration validation (5s)
2. meta dev - Launch tmux session (3s)
3. Navigate between panes with Ctrl+B + arrows (8s)
4. Zoom a pane to show full bacon TUI (3s)
5. Show pane numbers (2s)
6. Detach from session (3s)
7. Reattach to session (2s)
8. Stop a service (3s)
9. Exit cleanly (2s)

**Key Visual Elements:**
- Fast validation with meta doctor
- Instant tmux session launch
- Multiple bacon TUIs running simultaneously
- Turborepo dev servers running from workspace root
- Helpful navigation guide on startup
- Smooth keyboard navigation between panes
- Detach/reattach capability
- Professional multi-pane layout

### Detailed Recording Guide

See `DEMO_SCRIPT.md` for:
- Step-by-step recording instructions
- Timing guidelines for each action
- Tmux keyboard shortcuts reference
- Meta commands reference
- Tips for best results
- Troubleshooting

### Files Created

- `meta-demo.cast` - asciinema recording (shareable, embeddable)
- `meta-demo.gif` - Optimized GIF for social media (<10MB)

---

## Hashtag Strategy

**LinkedIn:**
Use 3-5 max, placed at the end:
#opensource #rust #webdev #developertools #monorepo

**X/Twitter:**
Use sparingly (1-2), only if relevant:
#rustlang #webdev

---

## Key Messaging Points - The Pivot Story

When discussing Meta v0.2.1, emphasize the learning journey:

### The Old Approach (What I Tried First)
- Built custom TUI to aggregate logs
- Tried to capture bacon's output
- Parse and display in a dashboard
- Lost bacon's interactivity

### The Realization
- Bacon IS a TUI orchestrator already
- Fighting it = bad architecture
- The real problem: orchestrating MULTIPLE bacon instances + turborepo

### The New Approach (v0.2.1)
- Use tmux for what it's good at: terminal multiplexing
- Give each process its own pane with full TTY
- Let bacon be bacon
- Meta just handles intelligent setup and routing

### Why This Is Better
1. **No Output Capture** - Processes run natively
2. **Full Bacon TUI** - All interactive features work
3. **Tool-Aware Routing** - Turbo from root, bacon from project dirs
4. **Professional UX** - Tmux is mature, battle-tested
5. **Detach/Reattach** - Don't lose your dev session

---

## Alternative Angles for Different Audiences

### For Rust Developers
"I was being an idiot, trying to replace bacon's TUI. v0.2.1 fixes that: multiple bacon instances orchestrated by tmux. Each with full interactive TUI. This is what I should have built from the start."

### For Full-Stack Teams
"v0.2.1 pivot: instead of custom TUI, we use tmux. Each Rust project gets bacon with full TUI, Next.js apps get turborepo. One `meta dev` command, everything just works."

### For Tool Builders
"Lesson learned: don't fight the ecosystem. Bacon is a great TUI. Tmux is a great multiplexer. Meta's job? Intelligent orchestration, not reinventing wheels."

### For Architecture Enthusiasts
"The v0.2.1 rewrite taught me: sometimes the 'clever' solution (custom TUI) loses to the simple one (tmux + native processes). Embrace existing tools, coordinate them well."

---

## Common Questions & Prepared Answers

**Q: Why change from custom TUI to tmux?**
A: The custom TUI broke bacon's interactivity. Bacon IS a TUI - it needs terminal ownership. Tmux gives each process its own terminal. Better architecture, simpler code, native UX.

**Q: Do I need to know tmux?**
A: Nope! Meta shows you the shortcuts when it starts. `Ctrl+B` + arrows = navigate. That's 90% of what you need. Plus it's a transferable skill.

**Q: What about the old TUI version?**
A: Dead. Removed completely in v0.2.1. The tmux approach is objectively better - no output parsing, full bacon interactivity, cleaner code.

**Q: Can I still use this without Rust/bacon?**
A: Yes! Meta orchestrates any tools. The bacon integration is just one use case. Works great for Next.js, Python, Go, whatever.

**Q: Is this production-ready?**
A: For local development orchestration, yes. v0.2.1 is solid. We're still adding features (watch mode, metrics) but the core works great.

---

## Update Announcement Framing

When announcing v0.2.1, be honest about the pivot:

✅ **Be Transparent:**
"I tried custom TUI first. It was wrong. Here's what I learned."

✅ **Show the Journey:**
"v0.1.0: Custom TUI (mistake)
v0.2.0: Realized output capture breaks bacon
v0.2.1: Complete rewrite with tmux (much better)"

✅ **Explain Why:**
"Fighting bacon = fighting the ecosystem. The right move: orchestrate multiple instances, don't replace them."

✅ **Invite Testing:**
"If you tried the old version, give v0.2.1 a shot. It's the approach I should have used from day one."

This transparency builds trust and shows you're willing to pivot when wrong.
