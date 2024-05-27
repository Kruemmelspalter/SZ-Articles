with import <nixpkgs> { };

let
    unstable = import (fetchTarball
      https://github.com/NixOS/nixpkgs/archive/nixos-unstable.tar.gz) {};
in
pkgs.mkShell {
    buildInputs = with pkgs; [
        rustup
        mold

        unstable.jetbrains.rust-rover
    ];
}
