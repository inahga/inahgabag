{ config, pkgs, ... }: {
  system.stateVersion = "23.05";
  imports = [
    # ./hardware-configuration.nix
    ./home-manager.nix
  ];

  # luks (w/ yubikey?)
  # sound
  # screen sharing
  # bluetooth
  # nmtui/wifi connection
  # notifications
  # updates
  # yubikey sudo/login
  # acpi command/battery
  # screen resolution management/display scaling
  # screenshots
  # screen locker
  # webcam
  # mozilla ublock origin
  # virtualization
  # rust dev

  boot = {
    loader.systemd-boot.enable = true;
    initrd.systemd.enable = true;
    kernelParams = [ "quiet" ];
    plymouth.enable = true;
  };

  networking.networkmanager.enable = true;
  hardware.bluetooth.enable = true;

  users.users.inahga = {
    isNormalUser = true;
    extraGroups = [ "wheel" "networkmanager" "libvirtd" ];
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
        command = "${pkgs.river}/bin/river";
        user = "inahga";
      };
    };
  };

  services.dbus.enable = true;
  services.flatpak.enable = true;
  services.blueman.enable = true;

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

  nixpkgs.config.allowUnfree = true;
  environment.defaultPackages = with pkgs; [
    acpi
    alacritty
    ansible
    brightnessctl
    chromium
    curl
    discord
    entr
    fd
    firefox-wayland
    fuzzel
    fwts
    fzf
    gcc
    git
    git-crypt
    grim
    kak-lsp
    kakoune
    kanshi
    kind
    krita
    libimobiledevice
    libnotify
    libreoffice
    mako
    moreutils
    mutt
    obs-studio
    pamixer
    pass
    playerctl
    pstree
    ripgrep
    river
    rsync
    shellcheck
    slack
    slurp
    socat
    spotify
    swaybg
    swayidle
    tmux
    usbutils
    valgrind
    vim
    virt-manager
    webcamoid
    wl-clipboard
    xdg-desktop-portal
    xterm
  ];

  # Set up firefox for wayland usage
  programs.firefox = {
    enable = true;
    package = pkgs.wrapFirefox pkgs.firefox-unwrapped {
      extraPolicies = { ExtensionSettings = { }; };
    };
  };

  xdg.portal = {
    enable = true;
    extraPortals = with pkgs; [ xdg-desktop-portal-wlr xdg-desktop-portal-gtk ];
  };

  virtualisation.podman = {
    enable = true;
    dockerCompat = true;
    defaultNetwork.settings.dns_enabled = true;
  };

  virtualisation.libvirtd.enable = true;
  programs.dconf.enable = true;
}
