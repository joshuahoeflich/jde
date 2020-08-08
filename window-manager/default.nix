{ pkgs ? import<nixpkgs> {} }:

pkgs.stdenv.mkDerivation {
  name = "dwm";
  buildInputs = with pkgs; [
    clang
    xorg.libXft
    xorg.libX11
    xorg.libXinerama
  ];
  prePatch = ''
    sed -i "s@/usr/local@$out@" config.mk
  '';
  src = pkgs.nix-gitignore.gitignoreSource [] ./.;
}
