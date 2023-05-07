{ config, pkgs, ... }:
let
  home-manager = builtins.fetchTarball
    "https://github.com/nix-community/home-manager/archive/master.tar.gz";
in {
  imports = [ (import "${home-manager}/nixos") ];

  home-manager.users.inahga = {
    home.stateVersion = "22.11";

    home.shellAliases = {
      fzf = "fzf --reverse";
      k = "kubectl";
      d = "docker";
      gs = "git status";
      gc = "git checkout";
      tf = "terraform";
      p = "podman";
    };

    services.mpris-proxy.enable = true;

    programs.git = {
      enable = true;
      userName = "Ameer Ghani";
      userEmail = if config.networking.hostName == "inahga-isrg-laptop"
      || config.networking.hostName == "inahga-isrg-desktop" then
        "idk@yet.com"
      else
        "inahga@gmail.com";
      ignores = [ "aghani*" "inahga*" ];
    };

    programs.bash = {
      enable = true;
      bashrcExtra = ''
        export GLADMOJI="😀😅😆😄😃😇😉😊🙂😋😍😘😜😝😛😎😏😻😺🙌💪👌🌞🔥👍💕💯✅🆒🆗💲"
        export SADMOJI="😶😳😠😞😡😕😣😖😫😩😮😱😨😰😯😦😢😥😥😵😭😴😷💀😿👎🙊💥🔪💔🆘⛔🚫❌🚷❓❗"
        export PROMPT_DIRTRIM=3
        export PS1="\n\e[36m\u@\h\[\e[0m\] \
        \$(if [ \$? == 0 ]; then echo -n \"\''${GLADMOJI:RANDOM%\''${#GLADMOJI}:1}\"; \
        else echo -n \"\''${SADMOJI:RANDOM%\''${#SADMOJI}:1}\"; fi)\
        \$(if [ -e ~/.git-prompt ]; then __git_ps1 \" (git: %s)\"; fi) \w\n🐧 "

        DEFAULT_TMUX="base"
        if [ -z "$TMUX" ]; then
            if tmux ls |& grep $DEFAULT_TMUX >/dev/null 2>&1; then
                tmux attach -t $DEFAULT_TMUX
            else
                tmux new-session -t $DEFAULT_TMUX
            fi
        fi

        gocover() {
            FILE=''${1:-coverage.out}
            go test -coverprofile "$FILE"
            go tool cover -html="$FILE"
        }

        if [ "$(uname -s)" == "Linux" ]; then
            alias open='xdg-open'
        fi

        alias kakoune='command kak'
        kak() {
            GITREPO=$(git rev-parse --show-toplevel 2>/dev/null)
            SESSION=general
            if [ -n "$GITREPO" ]; then
                SESSION=$(echo "$GITREPO" | md5sum | head -c6)
            fi

            if command kak -l | grep -q "$SESSION"; then
                command kak -c "$SESSION" "$@"
            else
                command kak -s "$SESSION" "$@"
            fi
        }

        alacritty-x11() {
            WINIT_UNIX_BACKEND=x11 alacritty "$@"
        }

        # alacritty pinebook pro fix
        # https://github.com/alacritty/alacritty/issues/128#issuecomment-663927477
        if [ "$(uname -m)" == "aarch64" ]; then
            export PAN_MESA_DEBUG=gl3
        fi
      '';
      initExtra = ''
        export XDG_DATA_DIRS=$XDG_DATA_DIRS:/usr/share:/var/lib/flatpak/exports/share:$HOME/.local/share/flatpak/exports/share
      '';
    };

    programs.firefox = { enable = true; };

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
    xdg.configFile."kak/kakrc".source = ./kakrc;
    xdg.configFile."kak-lsp/kak-lsp.toml".source = ./kak-lsp.toml;
    xdg.configFile."alacritty/alacritty.yml".source = ./alacritty.yml;
    home.file.".tmux.conf".source = ./tmux.conf;
    home.file.".vimrc".source = ./vimrc;
    home.file.".git-prompt".source = ./git-prompt.sh;

    gtk = {
      enable = true;
      theme = {
        name = "Materia-dark";
        package = pkgs.materia-theme;
      };
      iconTheme = {
        name = "BeautyLine";
        package = pkgs.beauty-line-icon-theme;
      };
    };

    dconf.settings = {
      "org/gnome/desktop/interface" = { color-scheme = "prefer-dark"; };
    };

    # better fuzzel theme
    # kanshi
  };
}
