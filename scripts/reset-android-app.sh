#!/bin/bash
# Usage: Only INTENDED to be used during DEVELOPMENT. This is intended to be used to clear the app data for the Aura app on Android.
# Package name
PACKAGE="com.dw3labs.aura"

# Check if ANDROID_HOME is set
if [ -z "$ANDROID_HOME" ]; then
    echo "Error: ANDROID_HOME environment variable is not defined."
    echo "Please set ANDROID_HOME to your Android SDK directory."
    exit 1
fi

# ADB path using ANDROID_HOME
ADB="$ANDROID_HOME/platform-tools/adb"

# Verify adb exists at the expected location
if [ ! -f "$ADB" ]; then
    echo "Error: adb not found at $ADB"
    exit 1
fi

echo "Using ADB at: $ADB"
echo "Clearing app data for $PACKAGE..."
$ADB shell pm clear $PACKAGE

if [ $? -eq 0 ]; then
    echo "Successfully cleared app data for $PACKAGE."
else
    echo "Failed to clear app data."
    exit 1
fi
