{ pkgs ? import <nixpkgs> { } }:

pkgs.stdenv.mkDerivation {
  name = "telebar_server";
  buildInputs = with pkgs; [ rustc cargo ];
  prePatch = ''
    sed -i "s@/usr/local@$out@" config.mk
  '';
  preConfigure = ''
    export HOME=$(mktemp -d)
  '';
  src = pkgs.nix-gitignore.gitignoreSource [ ] ./.;
}
