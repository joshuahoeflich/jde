{ pkgs ? import <nixpkgs> {}
, lib ? pkgs.lib
}:

let
  rev = "27877e7bcfa37b2c97a3dde170f870d4729ff807";
  sha256 = "1vyw8bpairxfxim931xg3pwyl3afh2mmjxa2i1igsaiaaxyssbc6";
in
(pkgs.emacs.override { srcRepo = true; }).overrideAttrs(old: rec {
  name = "emacs-${version}";
  version = builtins.substring 0 7 rev;
  src = pkgs.fetchFromGitHub {
    name  = "emacs-github-${rev}";
    owner = "emacs-mirror";
    repo = "emacs";
    rev = rev;
    sha256 = sha256;
  };
  patches = [
    ./clean-env.patch
    (lib.elemAt old.patches 1)
  ];
})
