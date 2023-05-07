{ config, pkgs, ... }: {
  system.stateVersion = "23.05";
  imports = [
    # ./hardware-configuration.nix
    ./home-manager.nix
  ];

  nixpkgs.config.allowUnfree = true;
  nix = {
    autoOptimiseStore = true;
    # nixPath = [
    #   "nixpkgs=/nix/var/nix/profiles/per-user/root/channels/nixos"
    #   "nixos-config=/etc/nixos/hosts/${variables.hostname}/default.nix"
    #   "/nix/var/nix/profiles/per-user/root/channels"
    # ];
  };

  boot = {
    loader.systemd-boot.enable = true;
    initrd.systemd.enable = true;
    kernelParams = [ "quiet" ];
    plymouth.enable = true;
  };

  swapDevices = [{
    device = "/swap";
    size = 8192;
  }];

  system.autoUpgrade.enable = true;

  networking.networkmanager = {
    enable = true;
    plugins = with pkgs; [ networkmanager-openvpn networkmanager-openconnect ];
  };
  networking.wireless.enable = true;

  hardware.bluetooth = {
    enable = true;
    settings = { General = { Enable = "Source,Sink,Media,Socket"; }; };
  };
  hardware.cpu.amd.updateMicrocode = true;

  time.timeZone = "America/Detroit";

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
  services.tlp.enable = true;
  services.fwupd.enable = true;

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
    acpi
    alacritty
    ansible
    awscli
    awscli2
    bind
    brightnessctl
    chromium
    clang-tools
    cobang
    coreutils
    curl
    dig
    discord
    drawio
    entr
    evince
    fd
    firefox-wayland
    flow
    fuzzel
    fwts
    fzf
    gcc
    git
    git-crypt
    go
    gopls
    grim
    htop
    jq
    k9s
    kak-lsp
    kakoune
    kanshi
    killall
    kind
    krita
    kubectl
    libimobiledevice
    libnotify
    libreoffice
    libuuid
    lvm2
    mako
    moreutils
    mutt
    obs-studio
    pamixer
    pass
    peek
    pgcli
    playerctl
    psmisc
    pstree
    ripgrep
    river
    rsync
    shellcheck
    shfmt
    slack
    slurp
    socat
    spotify
    swaybg
    swayidle
    terraform
    tmux
    tree
    unzip
    usbutils
    util-linux
    valgrind
    vim
    virt-manager
    vlc
    webcamoid
    wget
    wl-clipboard
    xdg-desktop-portal
    xterm
    yubikey-manager
    yubioath-flutter
    zip
    nixpkgs-fmt
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

  fonts.fonts = with pkgs; [
    noto-fonts
    noto-fonts-cjk
    noto-fonts-emoji
    liberation_ttf
    fira-code
    fira-code-symbols
    mplus-outline-fonts.githubRelease
    dina-font
    proggyfonts
    dejavu_fonts
  ];
}
