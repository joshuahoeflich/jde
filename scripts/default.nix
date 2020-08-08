{ pkgs ? import <nixpkgs> {} }:

pkgs.stdenv.mkDerivation {
  name = "personal-scripts";
  prePatch = ''
    sed -i "s@/usr/local@$out@" Makefile
  '';
  src = pkgs.nix-gitignore.gitignoreSource [] ./.;
}
