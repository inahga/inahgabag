#!/bin/sh
# Total rip from https://github.com/whereswaldon/configuration/blob/public/river/init

export XDG_CURRENT_DESKTOP=river

set_background() {
	riverctl background-color 0x2d2d2d
	riverctl border-color-focused 0x93a1a1
	riverctl border-color-unfocused 0x586e75
	swaybg -m fill -i "$(fd jpg "$HOME/Pictures/wallpapers" | shuf | head -n1)" &
}

set_environment() {
	# Ensure that the systemd user session has the necessary environment for
	# xdg-desktop-portal-wlr.
	dbus-update-activation-environment --systemd WAYLAND_DISPLAY XDG_CURRENT_DESKTOP="$XDG_CURRENT_DESKTOP"

	# Stop any services that are running, so that they receive the new env var when they restart.
	systemctl --user stop pipewire wireplumber xdg-desktop-portal xdg-desktop-portal-wlr
	systemctl --user start wireplumber

	riverctl set-repeat 50 300                                # set key repeat rate
	riverctl float-filter-add app-id float                    # type of window to float by default
	riverctl float-filter-add title "popup title with spaces" # title of window to float by default

	# riverctl csd-filter-add app-id "gedit" # apps that should use client-side decorations

	# Configure preferred touchpad stuff.
	# riverctl input "pointer-2-7-SynPS/2_Synaptics_TouchPad" tap enabled
	# riverctl input "pointer-2-7-SynPS/2_Synaptics_TouchPad" pointer-accel 0.3
}

set_bindings() {
	riverctl map normal Super Space spawn fuzzel                  # start launcher
	riverctl map normal Super Q close                             # close current view
	riverctl map normal Super+Shift E exit                        # exit river
	riverctl map normal Super Z toggle-fullscreen                 # toggle fullscreen view
	riverctl map normal Super F toggle-float                      # float window
	riverctl map normal Super H move left 100                     # move floating views
	riverctl map normal Super J move down 100                     #
	riverctl map normal Super K move up 100                       #
	riverctl map normal Super L move right 100                    #
	riverctl map normal Super+Mod1 H snap left                    # snap floating views
	riverctl map normal Super+Mod1 J snap down                    #
	riverctl map normal Super+Mod1 K snap up                      #
	riverctl map normal Super+Mod1 L snap right                   #
	riverctl map normal Super+Shift H resize horizontal -100      # resize views
	riverctl map normal Super+Shift J resize vertical 100         #
	riverctl map normal Super+Shift K resize vertical -100        #
	riverctl map normal Super+Shift L resize horizontal 100       #
	riverctl map normal Super Period focus-output next            # focus next monitor
	riverctl map normal Super Comma focus-output previous         # focus previous monitor
	riverctl map normal Super+Shift Period send-to-output next    # send to next monitor
	riverctl map normal Super+Shift Comma send-to-output previous # send to previous monitor
	riverctl map normal Super Return zoom                         # move focus view to top of layout stack
	riverctl map normal Super M focus-view next                   #
	riverctl map normal Super N focus-view previous               #
	riverctl map normal Super+Shift N swap next                   #
	riverctl map normal Super+Shift M swap previous               #
	riverctl map-pointer normal Super BTN_LEFT move-view          #
	riverctl map-pointer normal Super BTN_RIGHT resize-view       #

	# Ratio of the main view relative to the others.
	riverctl map normal Super+Shift Right send-layout-cmd rivertile "main-ratio -0.05"
	riverctl map normal Super+Shift Left send-layout-cmd rivertile "main-ratio +0.05"

	# Number of views in the main view.
	riverctl map normal Super+Shift Up send-layout-cmd rivertile "main-count +1"
	riverctl map normal Super+Shift Down send-layout-cmd rivertile "main-count -1"

	# Set where the main view is.
	riverctl map normal Super Up send-layout-cmd rivertile "main-location top"
	riverctl map normal Super Right send-layout-cmd rivertile "main-location right"
	riverctl map normal Super Down send-layout-cmd rivertile "main-location bottom"
	riverctl map normal Super Left send-layout-cmd rivertile "main-location left"

	# Configure screenshot shortcuts.
	riverctl map normal None Print spawn 'grim "$(xdg-user-dir PICTURES)"/"$(date +"%s_grim.png")"'
	riverctl map normal Control Print spawn 'grim -g "$(slurp)" "$(xdg-user-dir PICTURES)"/"$(date +"%s_grim.png")"'
	riverctl map normal Shift Print spawn 'grim - | wl-copy'
	riverctl map normal Shift+Control Print spawn 'grim -g "$(slurp)" - | wl-copy'

	# Declare a passthrough mode. This mode has only a single mapping to return to
	# normal mode. This makes it useful for testing a nested wayland compositor
	riverctl declare-mode passthrough
	riverctl map normal Super F11 enter-mode passthrough
	riverctl map passthrough Super F11 enter-mode normal

	# Set these to run in both normal and locked mode.
	for mode in normal locked; do
		riverctl map $mode None XF86AudioRaiseVolume spawn 'pamixer -i 5'
		riverctl map $mode None XF86AudioLowerVolume spawn 'pamixer -d 5'
		riverctl map $mode None XF86AudioMute spawn 'pamixer --toggle-mute'
		riverctl map $mode None XF86AudioMedia spawn 'playerctl play-pause'
		riverctl map $mode None XF86AudioPlay spawn 'playerctl play-pause'
		riverctl map $mode None XF86AudioPrev spawn 'playerctl previous'
		riverctl map $mode None XF86AudioNext spawn 'playerctl next'
		riverctl map $mode None XF86MonBrightnessUp spawn 'brightnessctl set +5%'
		riverctl map $mode None XF86MonBrightnessDown spawn 'brightnessctl set 5%-'
	done
}

spawn_daemons() {
	# idle timeout daemon
	swayidle -w timeout 1500 wlock before-sleep wlock 2>&1 |
		sed -e 's/^/swaylock: /' &
	sleep 0.1 && kanshi 2>&1 | sed -e 's/^/kanshi: /' & # display management daemon
	sleep 0.1 && mako 2>&1 | sed -e 's/^/mako: /' &     # notification daemone
}

set_background
set_bindings
set_environment
spawn_daemons

# Set and exec into the default layout generator, rivertile.
# River will send the process group of the init executable SIGTERM on exit.
riverctl default-layout rivertile
exec rivertile -view-padding 3 -outer-padding 3
