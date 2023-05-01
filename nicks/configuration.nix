{ config, pkgs, ... }: {
  system.stateVersion = "23.05";
  imports = [
    # ./hardware-configuration.nix
    ./home-manager.nix
  ];

  boot = {
    loader.systemd-boot.enable = true;
    initrd.systemd.enable = true;
    kernelParams = [ "quiet" ];
    plymouth.enable = true;
  };

  networking.networkmanager.enable = true;

  users.users.inahga = {
    isNormalUser = true;
    extraGroups = [ "wheel" "networkmanager" ];
    initialPassword = "password";
    openssh.authorizedKeys.keys = [
      "sk-ssh-ed25519@openssh.com AAAAGnNrLXNzaC1lZDI1NTE5QG9wZW5zc2guY29tAAAAIJBN1uV3RK41ghFQLYqNTVXUVALbn3KDr3E8HCxk7zC8AAAAEXNzaDp5dWJpa2V5LXNtYWxs"
      "sk-ssh-ed25519@openssh.com AAAAGnNrLXNzaC1lZDI1NTE5QG9wZW5zc2guY29tAAAAIHLtwflRB2PAPOAtqRpH5z7TJ/AG9iKo9pDUo/NdP0KyAAAAEXNzaDp5dWJpa2V5LXNtYWxs"
    ];
  };

  services.greetd = {
    enable = true;
    settings = {
      default_session = {
        command =
          "${pkgs.river}/bin/river -log-level debug >/home/inahga/river.log 2>&1";
        user = "inahga";
      };
    };
  };

  services.dbus.enable = true;

  services.pipewire = {
    enable = true;
    alsa.enable = true;
    alsa.support32Bit = true;
    pulse.enable = true;
  };

  security.rtkit.enable = true;
  security.doas = {
    enable = true;
    extraRules = [{
      groups = [ "wheel" ];
      keepEnv = true;
    }];
  };

  environment.defaultPackages = with pkgs; [
    fuzzel
    mako
    river
    swaybg
    swayidle
    xdg-desktop-portal

    chromium
    firefox

    libreoffice

    alacritty
    xterm
    tmux
    git
    rsync
    pstree

    vim
    kakoune
    kak-lsp
  ];
}
