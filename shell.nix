{ pkgs ? import <nixpkgs> {} }:

pkgs.mkShell {
  buildInputs = with pkgs; [
    shellcheck
    pre-commit
    socat
  ];
  shellHook = ''
    if [ -d "$PWD"/.git ] && [ ! -f "$PWD"/.git/hooks/pre-commit ]; then
      pre-commit install
    fi
  '';
}
