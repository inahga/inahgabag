{ config, pkgs, ... }:
let
  home-manager = builtins.fetchTarball
    "https://github.com/nix-community/home-manager/archive/master.tar.gz";
in {
  imports = [ (import "${home-manager}/nixos") ];

  home-manager.users.inahga = {
    home.stateVersion = "22.11";

    programs.git = {
      enable = true;
      userName = "Ameer Ghani";
      userEmail = if config.networking.hostName == "inahga-isrg-laptop"
      || config.networking.hostName == "inahga-isrg-desktop" then
        "idk@yet.com"
      else
        "inahga@gmail.com";
    };

    xdg.configFile."river/init" = {
      source = ./river-init.sh;
      executable = true;
    };

    home.file.".tmux.conf".source = ./tmux.conf;
    home.file.".bashrc" = {
      source = ./bashrc.sh;
      executable = true;
    };

    # kakrc
    # kak-lsp
  };
}
