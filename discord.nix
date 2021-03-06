{ pkgs ? import <nixpkgs> { config = { allowUnfree = true; }; } }:

let
  version = "0.0.12";
in
pkgs.discord.override {
  version = "${version}";
  src = pkgs.fetchurl {
    url = "https://dl.discordapp.net/apps/linux/${version}/discord-${version}.tar.gz";
    sha256 = "0qrzvc8cp8azb1b2wb5i4jh9smjfw5rxiw08bfqm8p3v74ycvwk8";
  };
}
