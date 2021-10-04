let
    sources = import ./nix/sources.nix { };
    pkgs = import sources.nixpkgs { overlays = [ (import sources.mozilla) ]; };
in
pkgs.mkShell {
    buildInputs = [
        (pkgs.rustChannelOf { date = "2021-10-03"; channel = "nightly"; }).rust
        pkgs.gcc
        pkgs.patchelf
    ];
}
