{ pkgs ? import <nixpkgs> { config = { allowUnfree = true; }; } }:

pkgs.buildEnv {
  name = "user-env";
  extraOutputsToInstall = [ "out" "bin" "lib" ];
  paths = with pkgs; [
    # Custom-nix packages
    (import ./window-manager/default.nix { }).outPath
    (import ./emacs/emacs.nix { }).outPath
    (import ./gcloud/google-cloud.nix {}).outPath
    (import ./scripts/default.nix {}).outPath

    # Binaries from the repos
    autocutsel
    bat
    bitwarden
    dash
    deepin.deepin-screenshot
    direnv
    dmenu
    exa
    feh
    gcc
    git
    go
    google-chrome
    htop
    kitty
    lf
    loccount
    lorri
    neofetch
    neovim
    nerdfonts
    nix
    nixfmt
    nodejs-12_x
    pamixer
    pavucontrol
    picom
    python38Full
    pywal
    ranger
    ripgrep
    rnix-lsp
    rustup
    spotify
    stack
    terraform-lsp
    unzip
    watchexec
    xclip
    xlockmore
    xmobar
    xmonad-with-packages
    zsh
  ];
}
