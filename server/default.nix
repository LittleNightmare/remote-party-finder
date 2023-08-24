let
    sources = import ./nix/sources.nix { };
    pkgs = import sources.nixpkgs { overlays = [ (import sources.mozilla) ]; };
    rust = pkgs.rustChannelOfTargets "nightly" "2022-12-02" [ "x86_64-unknown-linux-musl" ];
in
pkgs.mkShell {
    buildInputs = [
        rust
        pkgs.gcc
        pkgs.glibc
        pkgs.patchelf
        pkgs.musl
        pkgs.musl.dev
    ];
}
