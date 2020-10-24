{ pkgs ? import <nixpkgs> { config = { allowUnfree = true; }; } }:

let
  unstable = import (fetchTarball https://nixos.org/channels/nixos-unstable/nixexprs.tar.xz) {};
in
pkgs.buildEnv {
  name = "user-env";
  extraOutputsToInstall = [ "out" "bin" "lib" ];
  paths = with pkgs; [
    # Custom-nix packages
    (import ./dwm/default.nix {}).outPath
    # (import ./emacs/emacs.nix {}).outPath
    (import ./gcloud/google-cloud.nix {}).outPath
    (import ./scripts/default.nix {}).outPath
    (import ./polyblocks/default.nix {}).outPath
    (import ./discord.nix {})
    unstable.go
    unstable.emacs

    # Binaries from the repos
    autocutsel
    bat
    bind
    bitwarden
    brightnessctl
    bzip2
    dash
    deepin.deepin-screenshot
    direnv
    dmenu
    exa
    feh
    firefox
    gcc
    gimp
    git
    gitAndTools.hub
    gnome3.nautilus
    gvfs
    htop
    kdenlive
    kitty
    lf
    mpv
    mupdf
    ncdu
    neofetch
    neovim
    nerdfonts
    nix
    nix-direnv
    nixfmt
    nodePackages.nodemon
    nodejs-12_x
    okular
    pamixer
    pavucontrol
    picom
    python38Full
    pywal
    ranger
    ripgrep
    rnix-lsp
    simplescreenrecorder
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
