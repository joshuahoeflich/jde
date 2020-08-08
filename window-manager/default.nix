{ pkgs ? import<nixpkgs> {} }:

pkgs.stdenv.mkDerivation {
  name = "dwm";
  buildInputs = [
    (import ./shell.nix {}).buildInputs
  ];
  prePatch = ''
    sed -i "s@/usr/local@$out@" config.mk
  '';
  src = pkgs.nix-gitignore.gitignoreSource [] ./.;
}
