{ pkgs ? import <nixpkgs> { } }:

pkgs.rustPlatform.buildRustPackage {
  name = "polyblocks";
  src = pkgs.nix-gitignore.gitignoreSource [ ] ./.;
  nativeBuildInputs =
    [ pkgs.libpulseaudio pkgs.dbus pkgs.pkg-config pkgs.autoPatchelfHook ];
  cargoSha256 = "0vphnms5bpxcracwmp8dw3rymd2kpqfp6834pbs92iq2d6cv4rv7";
}
