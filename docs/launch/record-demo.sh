#!/bin/bash

# Script to record meta TUI demo with asciinema
# This will create a clean demo recording for social media

# Get the project root (3 levels up from docs/launch/)
PROJECT_ROOT="$(cd "$(dirname "$0")/../.." && pwd)"

echo "Starting meta TUI demo recording in 3 seconds..."
echo "The demo will:"
echo "  1. Start meta tui"
echo "  2. Navigate between services"
echo "  3. Filter logs for a specific service"
echo "  4. Show all logs"
echo "  5. Clear the buffer"
echo "  6. Quit"
echo ""
echo "Get ready to follow the prompts!"
sleep 3

# Record with asciinema
# -i 1 = capture input with 1 second idle time limit (makes recording faster)
# --title adds a title to the recording
cd "$PROJECT_ROOT"
asciinema rec \
  -i 1 \
  --title "Meta TUI - Rust Monorepo Orchestrator" \
  --command "bash -c 'meta tui'" \
  docs/launch/meta-demo.cast

echo ""
echo "Recording saved to docs/launch/meta-demo.cast"
echo ""
echo "Next steps:"
echo "  1. View it locally: asciinema play docs/launch/meta-demo.cast"
echo "  2. Convert to GIF: cd docs/launch && ./convert-to-gif.sh"
echo "  3. Upload to asciinema.org: asciinema upload docs/launch/meta-demo.cast"
