#!/bin/bash

# Usage: Only INTENDED to be used during DEVELOPMENT. This is intended to be used to clear the app data for the Aura app on iOS.
PACKAGE="com.dw3labs.aura"

# Get booted simulator
# Extracts UUID from output like "iPhone 14 (UUID) (Booted)"
DEVICE_ID=$(xcrun simctl list devices booted | grep "Booted" | head -n 1 | awk -F '[()]' '{print $2}' | tr -d '[:space:]')

if [ -z "$DEVICE_ID" ]; then
    echo "Error: No booted iOS simulator found."
    exit 1
fi

echo "Found booted simulator: $DEVICE_ID"

# Get data container path
DATA_PATH=$(xcrun simctl get_app_container "$DEVICE_ID" "$PACKAGE" data 2>/dev/null)

if [ -z "$DATA_PATH" ]; then
    echo "Error: Could not find data container for $PACKAGE. Is the app installed?"
    echo "This script only clears data for installed apps. If the app is not installed, there is no data to clear."
    exit 1
fi

echo "Clearing app data at: $DATA_PATH"

# Clear standard app data directories
rm -rf "$DATA_PATH/Documents/"*
rm -rf "$DATA_PATH/Library/"*
rm -rf "$DATA_PATH/tmp/"*

echo "Successfully cleared app data for $PACKAGE."
