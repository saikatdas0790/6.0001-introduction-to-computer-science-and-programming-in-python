let
  # Pinned nixpkgs, deterministic. Last updated: 2/12/21.
  pkgs = import (fetchTarball ("https://github.com/NixOS/nixpkgs/archive/cc45a3f8c98e1c33ca996e3504adefbf660a72d1.tar.gz")) { };

  # Rolling updates, not deterministic.
  # pkgs = import (fetchTarball("channel:nixpkgs-unstable")) {};
in
pkgs.mkShell {
  buildInputs = with pkgs; [ rustup ];

  shellHook = ''
    rustup toolchain install stable
  '';
}
