#!/bin/sh
set -e

OS_NAME=$(uname -s)
ARCH_NAME=$(uname -m)
OS=""
ARCH=""
INSTALL_DIR="/usr/local/bin"
NO_RELEASE_ASSET=""

# Calculate checksum. We use this function because Linux and macOS have different commands
calculate_checksum() {
    if [ "$OS_NAME" = "Darwin" ]; then
        # macOS
        shasum -a 256 "$1" | cut -d ' ' -f 1
    else
        # Linux and others
        sha256sum "$1" | cut -d ' ' -f 1
    fi
}

echo "Installing envfetch to $INSTALL_DIR"
echo "This script will activate sudo to install to $INSTALL_DIR"
sudo echo "Sudo activated"

if [ "$OS_NAME" = "Linux" ]; then
	OS="Linux"

    if [ "$ARCH_NAME" = "x86_64" ]; then
	    BUILD_TARGET="linux-amd64"
    elif [ "$ARCH_NAME" = "arm" ] || [ "$ARCH_NAME" = "arm64" ]; then
        BUILD_TARGET="linux-arm64"
    else
        NO_RELEASE_ASSET="true"
        echo "There is no release for this architecture: $ARCH_NAME" >&2
    fi
elif [ "$OS_NAME" = "Darwin" ]; then
	OS="macOS"

    if [ "$ARCH_NAME" = "x86_64" ]; then
	    BUILD_TARGET="darwin-amd64"
    elif [ "$ARCH_NAME" = "arm" ] || [ "$ARCH_NAME" = "arm64" ]; then
        BUILD_TARGET="darwin-arm64"
    else
        NO_RELEASE_ASSET="true"
        echo "There is no release for this architecture: $ARCH_NAME" >&2
    fi
else
	NO_RELEASE_ASSET="true"
	echo "There is no Unix release for this OS: $OS_NAME" >&2
fi

if [ "$NO_RELEASE_ASSET" ]; then
	exit 1
fi

if [ -f $INSTALL_DIR/envfetch ]; then
    echo "envfetch is already installed. Updating..."
fi

# Download file directly to install directory
sudo curl -sSL "https://github.com/ankddev/envfetch/releases/latest/download/envfetch-$BUILD_TARGET" --output "$INSTALL_DIR/envfetch"

# Check integrity
EXPECTED_CHECKSUM=$(curl -sSL "https://github.com/ankddev/envfetch/releases/latest/download/envfetch-$BUILD_TARGET.sha256" | tr -d '[:space:]' | tr -d '\n')
ACTUAL_CHECKSUM=$(calculate_checksum "$INSTALL_DIR/envfetch" | awk '{print $1}')

if [ "$EXPECTED_CHECKSUM" != "$ACTUAL_CHECKSUM" ]; then
    echo "Checksum mismatch" >&2
    echo "Expected: $EXPECTED_CHECKSUM"
    echo "Actual: $ACTUAL_CHECKSUM"
    echo "Please try again later or report this issue to the developer."
    exit 1
fi

# Give permissions for executable
sudo chmod +x "$INSTALL_DIR/envfetch"

echo "Successfully installed envfetch"
