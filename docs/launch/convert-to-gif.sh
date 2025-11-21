#!/bin/bash

# Convert asciinema recording to GIF
# This script will install agg (if needed) and convert the .cast file to GIF

# Get the script directory (docs/launch/)
SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
CAST_FILE="$SCRIPT_DIR/meta-demo.cast"
GIF_FILE="$SCRIPT_DIR/meta-demo.gif"

echo "Checking for agg..."

if ! command -v agg &> /dev/null; then
    echo "agg not found. Installing..."
    echo ""
    echo "Installing agg with cargo..."
    cargo install --git https://github.com/asciinema/agg

    if [ $? -ne 0 ]; then
        echo "Failed to install agg. Please install manually:"
        echo "  cargo install --git https://github.com/asciinema/agg"
        exit 1
    fi
fi

echo "agg is installed!"
echo ""

if [ ! -f "$CAST_FILE" ]; then
    echo "Error: $CAST_FILE not found!"
    echo "Please record a demo first with: ./record-demo.sh"
    exit 1
fi

echo "Converting $CAST_FILE to $GIF_FILE..."
echo ""

# Convert with optimized settings for social media
agg \
  --font-size 14 \
  --line-height 1.4 \
  --theme monokai \
  --speed 1.2 \
  --idle-time-limit 1.0 \
  --cols 100 \
  --rows 30 \
  "$CAST_FILE" \
  "$GIF_FILE"

if [ $? -eq 0 ]; then
    echo ""
    echo "✅ Success! GIF created: $GIF_FILE"
    echo ""

    # Check file size
    SIZE=$(ls -lh "$GIF_FILE" | awk '{print $5}')
    echo "File size: $SIZE"
    echo ""

    # Provide platform-specific guidance
    echo "Platform limits:"
    echo "  Twitter/X: Max 15MB"
    echo "  LinkedIn: Max 200MB (recommend <10MB)"
    echo ""

    # Check if we should optimize
    SIZE_BYTES=$(stat -f%z "$GIF_FILE" 2>/dev/null || stat -c%s "$GIF_FILE" 2>/dev/null)
    SIZE_MB=$((SIZE_BYTES / 1024 / 1024))

    if [ $SIZE_MB -gt 10 ]; then
        echo "⚠️  GIF is large (${SIZE_MB}MB). Consider optimizing:"
        echo ""
        echo "Option 1: Reduce frame rate and colors"
        echo "  agg --speed 1.5 --idle-time-limit 2.0 $CAST_FILE $GIF_FILE"
        echo ""
        echo "Option 2: Use gifsicle to optimize"
        echo "  brew install gifsicle"
        echo "  gifsicle -O3 --colors 256 $GIF_FILE -o ${GIF_FILE%.gif}-optimized.gif"
    else
        echo "✅ File size is good for social media!"
    fi

    echo ""
    echo "Preview your GIF:"
    echo "  open $GIF_FILE"
else
    echo "❌ Failed to convert. Please check the error above."
    exit 1
fi
