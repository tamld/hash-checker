#!/bin/bash
# GUI Automation Entrypoint
# Sets up headless environment for GUI tests

set -e

# Start Xvfb (virtual framebuffer)
Xvfb :99 -screen 0 1280x1024x24 &
XVFB_PID=$!

# Wait for X server to be ready
sleep 2

# Create XDG runtime directory
mkdir -p "$XDG_RUNTIME_DIR"
chmod 700 "$XDG_RUNTIME_DIR"

# Start dbus session
eval "$(dbus-launch --sh-syntax)"
export DBUS_SESSION_BUS_ADDRESS
export DBUS_SESSION_BUS_PID

# Cleanup handler
cleanup() {
    echo "Cleaning up..."
    kill $XVFB_PID 2>/dev/null || true
    kill $DBUS_SESSION_BUS_PID 2>/dev/null || true
}
trap cleanup EXIT

# Execute command
exec "$@"