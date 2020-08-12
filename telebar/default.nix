{ pkgs ? import <nixpkgs> { } }:

pkgs.stdenv.mkDerivation {
  name = "telebar_server";
  buildInputs = with pkgs; [ rustc cargo cacert curl ];
  prePatch = ''
    sed -i "s@/usr/local@$out@" config.mk
  '';
  preConfigure = ''
    curl https://github.com/rust-lang/crates.io-index
    export HOME=$(mktemp -d)
  '';
  src = pkgs.nix-gitignore.gitignoreSource [ ] ./.;
}
