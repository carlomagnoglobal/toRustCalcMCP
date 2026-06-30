#!/bin/bash
# Installation script for toRustCalcMCP
# Choose from three deployment options

set -e

REPO="carlomagnoglobal/toRustCalcMCP"
GITHUB_API="https://api.github.com/repos/$REPO"

# Color codes
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

print_header() {
    echo -e "${BLUE}╔════════════════════════════════════════╗${NC}"
    echo -e "${BLUE}║  toRustCalcMCP Installation Helper     ║${NC}"
    echo -e "${BLUE}╚════════════════════════════════════════╝${NC}"
    echo ""
}

print_success() {
    echo -e "${GREEN}✓ $1${NC}"
}

print_error() {
    echo -e "${RED}✗ $1${NC}"
}

print_info() {
    echo -e "${YELLOW}ℹ $1${NC}"
}

detect_platform() {
    OS=$(uname -s)
    ARCH=$(uname -m)

    case "$OS" in
        Darwin)
            case "$ARCH" in
                arm64|aarch64) PLATFORM="aarch64-apple-darwin" ;;
                x86_64) PLATFORM="x86_64-apple-darwin" ;;
                *) echo "Unsupported macOS architecture: $ARCH"; exit 1 ;;
            esac
            ;;
        Linux)
            case "$ARCH" in
                x86_64) PLATFORM="x86_64-unknown-linux-gnu" ;;
                aarch64) PLATFORM="aarch64-unknown-linux-gnu" ;;
                *) echo "Unsupported Linux architecture: $ARCH"; exit 1 ;;
            esac
            ;;
        *)
            echo "Unsupported OS: $OS"
            exit 1
            ;;
    esac

    echo "$PLATFORM"
}

show_menu() {
    echo ""
    echo -e "${BLUE}Choose installation method:${NC}"
    echo ""
    echo "  1) ${GREEN}Pre-built Binary${NC}    (Recommended - fastest, no dependencies)"
    echo "  2) ${GREEN}From Source (Cargo)${NC}  (Requires Rust, latest features)"
    echo "  3) ${GREEN}Docker${NC}              (Containerized, needs Docker)"
    echo ""
    echo -e "  0) ${RED}Exit${NC}"
    echo ""
    read -p "Select option (0-3): " choice
}

install_binary() {
    print_info "Installing pre-built binary..."
    echo ""

    PLATFORM=$(detect_platform)
    print_info "Detected platform: $PLATFORM"

    # Get latest release
    LATEST=$(curl -s "$GITHUB_API/releases/latest" | grep "tag_name" | cut -d'"' -f4)
    if [ -z "$LATEST" ]; then
        print_error "Could not fetch latest release information"
        return 1
    fi

    print_info "Latest version: $LATEST"

    # Construct download URL
    DOWNLOAD_URL="https://github.com/$REPO/releases/download/$LATEST/rcalc-$PLATFORM.tar.gz"
    TEMP_DIR=$(mktemp -d)

    print_info "Downloading from: $DOWNLOAD_URL"

    # Download
    if ! curl -L -o "$TEMP_DIR/rcalc.tar.gz" "$DOWNLOAD_URL" 2>/dev/null; then
        print_error "Failed to download binary"
        rm -rf "$TEMP_DIR"
        return 1
    fi

    # Extract
    tar xz -C "$TEMP_DIR" -f "$TEMP_DIR/rcalc.tar.gz"

    # Find the binary
    BINARY=$(find "$TEMP_DIR" -name "toRustCalcMCP" -type f | head -1)
    if [ -z "$BINARY" ]; then
        print_error "Binary not found in archive"
        rm -rf "$TEMP_DIR"
        return 1
    fi

    # Choose installation directory
    echo ""
    echo "Select installation directory:"
    echo "  1) /usr/local/bin (requires sudo)"
    echo "  2) ~/.local/bin (no sudo needed)"
    echo "  3) Custom path"
    read -p "Select (1-3): " dir_choice

    case "$dir_choice" in
        1)
            INSTALL_DIR="/usr/local/bin"
            SUDO="sudo"
            ;;
        2)
            INSTALL_DIR="$HOME/.local/bin"
            SUDO=""
            mkdir -p "$INSTALL_DIR"
            ;;
        3)
            read -p "Enter path: " INSTALL_DIR
            SUDO=""
            mkdir -p "$INSTALL_DIR"
            ;;
        *)
            print_error "Invalid choice"
            rm -rf "$TEMP_DIR"
            return 1
            ;;
    esac

    # Install
    echo ""
    print_info "Installing to $INSTALL_DIR..."
    $SUDO cp "$BINARY" "$INSTALL_DIR/toRustCalcMCP"
    $SUDO chmod +x "$INSTALL_DIR/toRustCalcMCP"

    # Cleanup
    rm -rf "$TEMP_DIR"

    # Verify
    if "$INSTALL_DIR/toRustCalcMCP" --mcp <<< '{"jsonrpc":"2.0","id":1,"method":"initialize","params":{}}' >/dev/null 2>&1; then
        print_success "Installation complete!"
        echo ""
        echo "Configuration for Claude:"
        echo '{
  "mcpServers": {
    "toRustCalcMCP": {
      "command": "'$INSTALL_DIR'/toRustCalcMCP",
      "args": ["--mcp"]
    }
  }
}'
        return 0
    else
        print_error "Verification failed"
        return 1
    fi
}

install_cargo() {
    print_info "Installing from source (Cargo)..."
    echo ""

    # Check Rust
    if ! command -v cargo &> /dev/null; then
        print_error "Rust/Cargo not installed"
        echo ""
        echo "Install Rust from https://rustup.rs/:"
        echo "  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh"
        return 1
    fi

    RUST_VERSION=$(rustc --version)
    print_info "Using: $RUST_VERSION"
    echo ""

    # Clone or update
    CLONE_PATH="$HOME/.toRustCalcMCP"
    if [ -d "$CLONE_PATH" ]; then
        print_info "Updating existing clone in $CLONE_PATH..."
        cd "$CLONE_PATH"
        git pull
    else
        print_info "Cloning repository..."
        git clone https://github.com/$REPO.git "$CLONE_PATH"
        cd "$CLONE_PATH"
    fi

    print_info "Building release binary..."
    cargo build --release --bin toRustCalcMCP

    print_success "Build complete!"
    echo ""
    echo "Configuration for Claude:"
    echo '{
  "mcpServers": {
    "toRustCalcMCP": {
      "command": "cargo",
      "args": ["run", "--release", "--bin", "toRustCalcMCP", "--", "--mcp"],
      "cwd": "'$CLONE_PATH'"
    }
  }
}'
    return 0
}

install_docker() {
    print_info "Setting up Docker installation..."
    echo ""

    # Check Docker
    if ! command -v docker &> /dev/null; then
        print_error "Docker not installed"
        echo ""
        echo "Install Docker from https://www.docker.com/products/docker-desktop"
        return 1
    fi

    DOCKER_VERSION=$(docker --version)
    print_info "Using: $DOCKER_VERSION"
    echo ""

    print_info "Pulling latest image..."
    docker pull ghcr.io/$REPO:latest

    print_success "Installation complete!"
    echo ""
    echo "Configuration for Claude:"
    echo '{
  "mcpServers": {
    "toRustCalcMCP": {
      "command": "docker",
      "args": ["run", "-i", "--rm", "ghcr.io/'$REPO':latest"]
    }
  }
}'

    return 0
}

test_installation() {
    echo ""
    read -p "Test installation now? (y/n): " test_choice

    if [ "$test_choice" = "y" ] || [ "$test_choice" = "Y" ]; then
        print_info "Running test..."
        if toRustCalcMCP --mcp <<< '{"jsonrpc":"2.0","id":1,"method":"initialize","params":{}}' | grep -q "protocolVersion"; then
            print_success "Installation test passed!"
        else
            print_error "Installation test failed"
            return 1
        fi
    fi

    return 0
}

main() {
    print_header

    while true; do
        show_menu

        case "$choice" in
            1) install_binary && test_installation; break ;;
            2) install_cargo && test_installation; break ;;
            3) install_docker && test_installation; break ;;
            0) echo "Exiting..."; exit 0 ;;
            *) print_error "Invalid choice"; continue ;;
        esac
    done

    echo ""
    print_success "Setup complete!"
    echo ""
    echo "Next steps:"
    echo "  1. Copy the configuration shown above"
    echo "  2. Add it to your Claude configuration"
    echo "  3. Restart Claude"
    echo ""
    echo "For more information, see:"
    echo "  - DEPLOYMENT.md - Detailed deployment guide"
    echo "  - QUICKSTART.md - Quick start guide"
    echo "  - README.md - Full documentation"
}

main
