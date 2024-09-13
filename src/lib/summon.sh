#!/bin/bash
open_url="https://cal.com/sideral"
if command -v xdg-open >/dev/null 2>&1; then
    xdg-open "$open_url"
elif command -v gnome-open >/dev/null 2>&1; then
    gnome-open "$open_url"
elif command -v open >/dev/null 2>&1; then
    open "$open_url"
else
    echo "Could not detect the web browser to use. Open URL manually: $open_url"
fi