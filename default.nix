let
    sources = import ./nix/sources.nix { };
    pkgs = import sources.nixpkgs { overlays = [ (import sources.mozilla) ]; };
in
pkgs.mkShell {
    buildInputs = [
        (pkgs.rustChannelOf { date = "2021-09-20"; channel = "nightly"; }).rust
        pkgs.gcc
        pkgs.patchelf
    ];
}
