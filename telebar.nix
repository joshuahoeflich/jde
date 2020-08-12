{ pkgs ? import <nixpkgs> { } }:

pkgs.rustPlatform.buildRustPackage {
  name = "telebar_server";
  src = ./telebar;
  checkPhase = "";
  cargoSha256 = "1cw6120xgbn4mvx6wjlv5cmmac8gmym40r6wz3lrv7av2s42mhnw";
}
