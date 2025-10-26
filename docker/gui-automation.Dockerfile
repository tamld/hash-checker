# GUI Automation Container
# Purpose: Isolated environment for headless GUI testing without polluting host
# Usage: docker build -f docker/gui-automation.Dockerfile -t hash-checker-gui-automation .

FROM rust:1.88-slim

# Install GUI test dependencies (minimal)
RUN apt-get update && apt-get install -y \
    # Rust build essentials
    pkg-config \
    libasound2-dev \
    # XDG portal backend (no GTK runtime needed)
    xdg-desktop-portal \
    # Headless testing
    xvfb \
    dbus-x11 \
    # GTK4 (optional, feature-gated)
    libgtk-4-dev \
    libadwaita-1-dev \
    libglib2.0-dev \
    # Python for analysis scripts
    python3 \
    python3-pip \
    # Cleanup
    && rm -rf /var/lib/apt/lists/*

# Install Python analysis tools
COPY scripts/requirements-automation.txt /tmp/
RUN pip3 install --no-cache-dir --break-system-packages -r /tmp/requirements-automation.txt

# Set up Rust environment
ENV CARGO_HOME=/usr/local/cargo
ENV PATH="${CARGO_HOME}/bin:${PATH}"

# Configure headless display
ENV DISPLAY=:99
ENV XDG_RUNTIME_DIR=/tmp/xdg-runtime

# Working directory
WORKDIR /workspace

# Entrypoint: start xvfb and dbus session
COPY docker/gui-automation-entrypoint.sh /entrypoint.sh
RUN chmod +x /entrypoint.sh

ENTRYPOINT ["/entrypoint.sh"]
CMD ["bash"]