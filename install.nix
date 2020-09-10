{ pkgs ? import <nixpkgs> { config = { allowUnfree = true; }; } }:

pkgs.buildEnv {
  name = "user-env";
  extraOutputsToInstall = [ "out" "bin" "lib" ];
  paths = with pkgs; [
    # Custom-nix packages
    (import ./dwm/default.nix {}).outPath
    (import ./emacs/emacs.nix {}).outPath
    (import ./gcloud/google-cloud.nix {}).outPath
    (import ./scripts/default.nix {}).outPath
    (import ./polyblocks/default.nix {}).outPath
    (import ./discord.nix {})

    # Binaries from the repos
    autocutsel
    bat
    bind
    bitwarden
    brightnessctl
    dash
    deepin.deepin-screenshot
    direnv
    # discord_12
    dmenu
    exa
    feh
    gcc
    gimp
    git
    gitAndTools.hub
    gnome3.nautilus
    google-chrome
    gvfs
    htop
    i3blocks
    kitty
    lf
    ncdu
    neofetch
    neovim
    nerdfonts
    nix
    nix-direnv
    nixfmt
    nodejs-12_x
    pamixer
    pavucontrol
    picom
    python38Full
    pywal
    racket
    ranger
    ripgrep
    rnix-lsp
    socat
    spotify
    stack
    steam
    terraform-lsp
    tokei
    unzip
    watchexec
    xclip
    xlockmore
    xorg.xev
    zoom-us
    zsh
  ];
}
