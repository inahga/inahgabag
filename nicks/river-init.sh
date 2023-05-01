#!/bin/sh
# Total rip from https://github.com/whereswaldon/configuration/blob/public/river/init

# Set background and border color
riverctl background-color 0x000000
riverctl border-color-focused 0x93a1a1
riverctl border-color-unfocused 0x586e75

# swaybg -m fill -i "$(fd jpg "$HOME/Pictures/wallpapers" | shuf | head -n1)" &

riverctl map normal Super Space spawn fuzzel # start launcher
riverctl map normal Super Q close            # close current view
riverctl map normal Super+Shift E exit       # exit river

# Mod+J and Mod+K to focus the next/previous view in the layout stack
riverctl map normal Super J focus-view next
riverctl map normal Super K focus-view previous

# Mod+Shift+J and Mod+Shift+K to swap the focused view with the next/previous
# view in the layout stack
riverctl map normal Super+Shift J swap next
riverctl map normal Super+Shift K swap previous

# Mod+Period and Mod+Comma to focus the next/previous output
riverctl map normal Super Period focus-output next
riverctl map normal Super Comma focus-output previous

# Mod+Shift+{Period,Comma} to send the focused view to the next/previous output
riverctl map normal Super+Shift Period send-to-output next
riverctl map normal Super+Shift Comma send-to-output previous

# Mod+Return to bump the focused view to the top of the layout stack
riverctl map normal Super Return zoom

# Mod+H and Mod+L to decrease/increase the main ratio of rivertile(1)
riverctl map normal Super H send-layout-cmd rivertile "main-ratio -0.05"
riverctl map normal Super L send-layout-cmd rivertile "main-ratio +0.05"

# Mod+Shift+H and Mod+Shift+L to increment/decrement the main count of rivertile(1)
riverctl map normal Super+Shift H send-layout-cmd rivertile "main-count +1"
riverctl map normal Super+Shift L send-layout-cmd rivertile "main-count -1"

# Mod+Alt+{H,J,K,L} to move views
riverctl map normal Super+Mod1 H move left 100
riverctl map normal Super+Mod1 J move down 100
riverctl map normal Super+Mod1 K move up 100
riverctl map normal Super+Mod1 L move right 100

# Mod+Alt+Control+{H,J,K,L} to snap views to screen edges
riverctl map normal Super+Mod1+Control H snap left
riverctl map normal Super+Mod1+Control J snap down
riverctl map normal Super+Mod1+Control K snap up
riverctl map normal Super+Mod1+Control L snap right

# Mod+Alt+Shif+{H,J,K,L} to resize views
riverctl map normal Super+Mod1+Shift H resize horizontal -100
riverctl map normal Super+Mod1+Shift J resize vertical 100
riverctl map normal Super+Mod1+Shift K resize vertical -100
riverctl map normal Super+Mod1+Shift L resize horizontal 100

# Mod + Left Mouse Button to move views
riverctl map-pointer normal Super BTN_LEFT move-view

# Mod + Right Mouse Button to resize views
riverctl map-pointer normal Super BTN_RIGHT resize-view

for i in $(seq 1 9); do
	tags=$((1 << ($i - 1)))

	# Mod+[1-9] to focus tag [0-8]
	riverctl map normal Super $i set-focused-tags $tags

	# Mod+Shift+[1-9] to tag focused view with tag [0-8]
	riverctl map normal Super+Shift $i set-view-tags $tags

	# Mod+Ctrl+[1-9] to toggle focus of tag [0-8]
	riverctl map normal Super+Control $i toggle-focused-tags $tags

	# Mod+Shift+Ctrl+[1-9] to toggle tag [0-8] of focused view
	riverctl map normal Super+Shift+Control $i toggle-view-tags $tags
done

# Mod+0 to focus all tags
# Mod+Shift+0 to tag focused view with all tags
all_tags=$(((1 << 32) - 1))
riverctl map normal Super 0 set-focused-tags $all_tags
riverctl map normal Super+Shift 0 set-view-tags $all_tags

# Mod+Space to toggle float
# riverctl map normal Super Space toggle-float

# Mod+F to toggle fullscreen
riverctl map normal Super Z toggle-fullscreen

# Mod+{Up,Right,Down,Left} to change layout orientation
riverctl map normal Super Up send-layout-cmd rivertile "main-location top"
riverctl map normal Super Right send-layout-cmd rivertile "main-location right"
riverctl map normal Super Down send-layout-cmd rivertile "main-location bottom"
riverctl map normal Super Left send-layout-cmd rivertile "main-location left"

# Configure screenshot shortcuts.
# riverctl map normal None Print spawn 'grim "$(xdg-user-dir PICTURES)"/"$(date +"%s_grim.png")"'
# riverctl map normal Control Print spawn 'grim -g "$(slurp)" "$(xdg-user-dir PICTURES)"/"$(date +"%s_grim.png")"'
# riverctl map normal Shift Print spawn 'grim - | wl-copy'
# riverctl map normal Shift+Control Print spawn 'grim -g "$(slurp)" - | wl-copy'

# Declare a passthrough mode. This mode has only a single mapping to return to
# normal mode. This makes it useful for testing a nested wayland compositor
riverctl declare-mode passthrough
riverctl map normal Super F11 enter-mode passthrough
riverctl map passthrough Super F11 enter-mode normal

# Various media key mapping examples for both normal and locked mode which do
# not have a modifier
for mode in normal locked; do
	# Control pulse audio volume with pamixer (https://github.com/cdemoulins/pamixer)
	riverctl map $mode None XF86AudioRaiseVolume spawn 'pamixer -i 5'
	riverctl map $mode None XF86AudioLowerVolume spawn 'pamixer -d 5'
	riverctl map $mode None XF86AudioMute spawn 'pamixer --toggle-mute'

	# Control MPRIS aware media players with playerctl (https://github.com/altdesktop/playerctl)
	riverctl map $mode None XF86AudioMedia spawn 'playerctl play-pause'
	riverctl map $mode None XF86AudioPlay spawn 'playerctl play-pause'
	riverctl map $mode None XF86AudioPrev spawn 'playerctl previous'
	riverctl map $mode None XF86AudioNext spawn 'playerctl next'

	# Control screen backlight brighness with light (https://github.com/haikarainen/light)
	riverctl map $mode None XF86MonBrightnessUp spawn 'light -A 5'
	riverctl map $mode None XF86MonBrightnessDown spawn 'light -U 5'
done

# Set keyboard repeat rate
riverctl set-repeat 50 300

# Make certain views start floating
riverctl float-filter-add app-id float
riverctl float-filter-add title "popup title with spaces"

# Set app-ids and titles of views which should use client side decorations
riverctl csd-filter-add app-id "gedit"

# Set and exec into the default layout generator, rivertile.
# River will send the process group of the init executable SIGTERM on exit.
riverctl default-layout rivertile

# Configure preferred touchpad stuff.
# riverctl input "pointer-2-7-SynPS/2_Synaptics_TouchPad" tap enabled
# riverctl input "pointer-2-7-SynPS/2_Synaptics_TouchPad" pointer-accel 0.3

# Launch idle management process that will lock screen.
swayidle -w \
	timeout 1500 wlock \
	before-sleep wlock 2>&1 | sed -e 's/^/swaylock: /' &

# Launch display management daemon.
# sleep 0.1 && kanshi 2>&1 | sed -e 's/^/kanshi: /' &

# Launch notification daemon.
sleep 0.1 && mako 2>&1 | sed -e 's/^/mako: /' &

# Ensure that the systemd user session has the necessary environment for
# xdg-desktop-portal-wlr.
export XDG_CURRENT_DESKTOP=river
dbus-update-activation-environment --systemd WAYLAND_DISPLAY XDG_CURRENT_DESKTOP="$XDG_CURRENT_DESKTOP"

# Stop any services that are running, so that they receive the new env var when they restart.
systemctl --user stop pipewire wireplumber xdg-desktop-portal xdg-desktop-portal-wlr
systemctl --user start wireplumber

exec rivertile -view-padding 2 -outer-padding 2
