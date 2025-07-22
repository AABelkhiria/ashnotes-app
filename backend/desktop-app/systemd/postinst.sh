#!/bin/sh
set -e

if [ -n "$SUDO_USER" ] && [ "$SUDO_USER" != "root" ]; then
    # Get the UID of the user who ran sudo
    uid=$(id -u "$SUDO_USER")
    if ! pgrep -u "$uid" -n systemd >/dev/null; then
        echo "User systemd process not found. Skipping service setup."
        exit 0
    fi

    # Find the D-Bus session address for the user's session
    dbus_address=$(grep -z DBUS_SESSION_BUS_ADDRESS "/proc/$(pgrep -u "$uid" -n systemd)/environ" | cut -d= -f2-)

    if [ -n "$dbus_address" ]; then
        # Run the systemctl commands as the user, with the correct D-Bus address
        sudo -u "$SUDO_USER" DBUS_SESSION_BUS_ADDRESS="$dbus_address" systemctl --user daemon-reload
        sudo -u "$SUDO_USER" DBUS_SESSION_BUS_ADDRESS="$dbus_address" systemctl --user enable ashnotes.service
        sudo -u "$SUDO_USER" DBUS_SESSION_BUS_ADDRESS="$dbus_address" systemctl --user start ashnotes.service
    else
        echo "Could not determine user's D-Bus session address. Please run 'systemctl --user enable --now ashnotes.service' manually."
    fi
fi

exit 0
