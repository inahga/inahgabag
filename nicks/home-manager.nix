{ config, pkgs, ... }:
let
  home-manager = builtins.fetchTarball
    "https://github.com/nix-community/home-manager/archive/master.tar.gz";
in {
  imports = [ (import "${home-manager}/nixos") ];

  home-manager.users.inahga = {
    home.stateVersion = "22.11";

    services.mpris-proxy.enable = true;

    programs.git = {
      enable = true;
      userName = "Ameer Ghani";
      userEmail = if config.networking.hostName == "inahga-isrg-laptop"
      || config.networking.hostName == "inahga-isrg-desktop" then
        "idk@yet.com"
      else
        "inahga@gmail.com";
    };

    programs.bash = {
      enable = true;
      bashrcExtra = ""; # TODO
      initExtra = ''
        export XDG_DATA_DIRS=$XDG_DATA_DIRS:/usr/share:/var/lib/flatpak/exports/share:$HOME/.local/share/flatpak/exports/share
      '';
    };

    home.sessionVariables = {
      MOZ_ENABLE_WAYLAND = 1;
      XDG_CURRENT_DESKTOP = "river";
      XKB_DEFAULT_OPTIONS = "caps:escape";
      EDITOR = "kak";
    };

    xdg.configFile."river/init" = {
      source = ./river-init.sh;
      executable = true;
    };

    home.file.".tmux.conf".source = ./tmux.conf;

    # kakrc
    # kak-lsp
    # vimrc
    # gitignore
    # dconf??
    # better fuzzel theme
    # kanshi
    # dark mode
  };
}
