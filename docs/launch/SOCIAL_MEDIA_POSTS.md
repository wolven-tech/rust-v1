# Social Media Launch Posts

## LinkedIn Version

---

I spent 6 months wrestling with monorepo tooling.

Running multiple services meant:
- 5 terminal tabs open
- Logs mixed together
- No idea which service failed
- 10+ commands just to start dev

So I built something better.

**Introducing Meta** - a Rust-powered orchestrator for modern monorepos.

One command. One dashboard. Real-time color-coded logs.

```bash
meta tui
```

That's it.

Here's what makes it different:

🎨 **Color-coded logs** - Spot errors instantly (red = bad, white = info)
🔍 **Smart filtering** - Focus on one service with a keypress
⚡ **Blazingly fast** - 5MB memory, 3000+ lines/sec throughput
🎯 **Zero config** - Works out of the box
📊 **TUI dashboard** - Visual feedback for all your services

The best part?

It's not just for Rust projects. Works with Next.js, Node, Python, anything.

We've open-sourced the full monorepo template:
- Rust API (Axum + Clean Architecture)
- Next.js apps (Marketing + Application)
- Meta orchestrator with TUI
- 8 shared packages ready to use

⚠️ **Early access warning:** Meta just hit v0.2.0. The core is solid, but we're still adding features. Use it, break it, tell us what's missing.

Check it out: https://github.com/wolven-tech/rust-v1

Quick start:
```bash
bunx degit wolven-tech/rust-v1 my-project
cd my-project && bun install
cd tooling/meta && ./install.sh && cd ../..
meta tui
```

What's your biggest pain point with monorepo development?

(Drop a comment - I read every one)

---

## X (Twitter) Version - Thread

---

**Tweet 1:**
I spent 6 months fighting monorepo tooling hell.

Multiple services = terminal chaos.

So I built Meta - a Rust-powered orchestrator with a TUI dashboard.

One command to rule them all:
`meta tui`

Just dropped v0.2.0 ↓

---

**Tweet 2:**
What makes Meta different:

🎨 Color-coded logs (spot errors instantly)
🔍 Filter by service (press Enter)
⚡ 5MB memory, 3000+ lines/sec
🎯 Zero config
📊 Real-time TUI dashboard

All in one ~3MB binary.

---

**Tweet 3:**
It's not just for Rust.

Works with:
- Next.js / Node
- Python / Django
- Go services
- Anything with a CLI

If it runs in a terminal, Meta can orchestrate it.

---

**Tweet 4:**
We open-sourced the full template:

✅ Rust API (Axum)
✅ Next.js apps
✅ Meta orchestrator
✅ 8 shared packages
✅ Production-ready

Scaffold in 30 seconds:
`bunx degit wolven-tech/rust-v1 my-project`

https://github.com/wolven-tech/rust-v1

---

**Tweet 5:**
⚠️ Fair warning:

Meta just hit v0.2.0. Core is solid, but we're still adding features.

Use it. Break it. Tell us what's missing.

Early adopters shape the roadmap.

What feature would you want first?

---

## X (Twitter) Version - Single Tweet

---

Just shipped Meta v0.2.0 - a Rust-powered monorepo orchestrator with TUI dashboard

One command. One view. All your services.

🎨 Color-coded logs
🔍 Smart filtering
⚡ 5MB memory
📊 Real-time updates

Open source template: https://github.com/wolven-tech/rust-v1

⚠️ Early access - help us shape it

---

## Instagram/Visual Platform Caption

---

Meta v0.2.0 is live 🚀

Your monorepo command center:
→ One TUI dashboard
→ Color-coded logs
→ Real-time filtering
→ Built in Rust

`meta tui` and you're done.

⚠️ Just released - early adopters welcome

Open source at github.com/wolven-tech/rust-v1

What's your monorepo pain point? 👇

#rust #webdev #opensource #developer #programming #monorepo #devtools

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

### What Gets Recorded (20-30 seconds)

1. Meta TUI loading (3s)
2. Navigate between services with arrow keys (5s)
3. Filter logs for one service with Enter (3s)
4. Show all logs with 'a' (2s)
5. Clear buffer with 'c' (2s)
6. Navigate again to show responsiveness (2s)
7. Quit gracefully with 'q' (2s)

**Key Visual Elements:**
- Fast startup time
- Color-coded logs (red errors, white info)
- Smooth keyboard navigation
- Real-time streaming
- Professional UI

### Detailed Recording Guide

See `DEMO_SCRIPT.md` for:
- Step-by-step recording instructions
- Timing guidelines for each action
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

**Instagram:**
Use more (10-15):
#rust #webdev #opensource #programming #developer #devtools #monorepo #nextjs #typescript #fullstack #coding #softwaredevelopment #tech #buildInpublic #indiedev
