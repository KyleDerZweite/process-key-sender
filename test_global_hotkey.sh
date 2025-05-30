#!/bin/bash

# Test script for Global Hotkey Feature
# This script demonstrates the new global hotkey functionality

echo "🚀 Testing Global Hotkey Feature"
echo "================================="
echo ""

# Create a simple test configuration
cat > test_config.json << EOF
{
  "process_name": "bash",
  "pause_hotkey": "ctrl+alt+r",
  "key_sequence": [
    {
      "key": "space",
      "interval_after": "2000ms"
    }
  ],
  "max_retries": 3,
  "verbose": true,
  "loop_sequence": true,
  "repeat_count": 0,
  "restore_focus": true
}
EOF

echo "📝 Created test configuration:"
cat test_config.json
echo ""

echo "🔧 Building the project..."
cargo build --release

if [ $? -eq 0 ]; then
    echo "✅ Build successful!"
    echo ""
    echo "🎯 To test the global hotkey feature:"
    echo "1. Run: ./target/release/pks --config test_config.json"
    echo "2. Wait for the automation to start"
    echo "3. Switch to any other application window"
    echo "4. Press Ctrl+Alt+R to pause/resume globally"
    echo "5. Observe the pause/resume messages in terminal"
    echo "6. Press Ctrl+C to stop"
    echo ""
    echo "🔥 Expected behavior:"
    echo "   - Hotkey works regardless of window focus"
    echo "   - Clear pause/resume feedback in console"
    echo "   - Automation respects pause state"
    echo ""
    
    # Cleanup
    rm -f test_config.json
else
    echo "❌ Build failed!"
    exit 1
fi
