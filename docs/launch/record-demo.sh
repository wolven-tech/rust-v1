#!/bin/bash

# Script to record meta tmux demo with asciinema
# This will create a clean demo recording for social media

# Get the project root (3 levels up from docs/launch/)
PROJECT_ROOT="$(cd "$(dirname "$0")/../.." && pwd)"

echo "Starting meta tmux demo recording in 3 seconds..."
echo ""
echo "The demo will show:"
echo "  1. meta doctor - Configuration validation"
echo "  2. meta dev - Launch tmux session with all services"
echo "  3. Navigate between panes (Ctrl+B then arrows)"
echo "  4. Zoom/unzoom a pane (Ctrl+B then Z)"
echo "  5. Detach from session (Ctrl+B then D)"
echo "  6. Reattach to session"
echo "  7. Exit cleanly"
echo ""
echo "Follow the detailed steps in DEMO_SCRIPT.md"
echo "Get ready!"
sleep 3

# Record with asciinema
# -i 1 = capture input with 1 second idle time limit (makes recording faster)
# --title adds a title to the recording
cd "$PROJECT_ROOT"

echo ""
echo "Recording will start now..."
echo "Follow these commands in order:"
echo "  1. meta doctor"
echo "  2. meta dev"
echo "  3. Use tmux navigation (see DEMO_SCRIPT.md for details)"
echo "  4. Exit: Ctrl+B then D, or close panes with Ctrl+C"
echo ""
echo "Press Enter when ready to start recording..."
read -r

asciinema rec \
  -i 1 \
  --title "Meta - Rust Monorepo Orchestrator with Tmux" \
  docs/launch/meta-demo.cast

echo ""
echo "Recording saved to docs/launch/meta-demo.cast"
echo ""
echo "Next steps:"
echo "  1. View it locally: asciinema play docs/launch/meta-demo.cast"
echo "  2. Convert to GIF: cd docs/launch && ./convert-to-gif.sh"
echo "  3. Upload to asciinema.org: asciinema upload docs/launch/meta-demo.cast"
echo ""
echo "Tips:"
echo "  - If you're not happy with the recording, just run ./record-demo.sh again"
echo "  - See DEMO_SCRIPT.md for detailed step-by-step instructions"
echo "  - Practice the tmux keyboard shortcuts before recording"
