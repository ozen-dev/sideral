#!/bin/bash
open_url="https://cal.com/sideral"
if which xdg-open > /dev/null; then
    xdg-open "$open_url"
elif which gnome-open > /dev/null; then
    gnome-open "$open_url"
elif which open > /dev/null; then
    open "$open_url"
else
    echo "Could not detect the web browser to use. Open URL manually: https://cal.com/sideral"
fi
