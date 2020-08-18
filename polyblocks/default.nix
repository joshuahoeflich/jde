{ pkgs ? import <nixpkgs> { } }:

pkgs.rustPlatform.buildRustPackage {
  name = "polyblocks";
  src = pkgs.nix-gitignore.gitignoreSource [ ] ./.;
  nativeBuildInputs =
    [ pkgs.libpulseaudio pkgs.dbus pkgs.pkg-config pkgs.autoPatchelfHook ];
  cargoSha256 = "13p6j01kh2qb41p1c6l9d48ghmwzlfp8zm9hlkq1wjj4zkja0x2i";
}
