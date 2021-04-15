let
  sources = import ./nix/sources.nix;
in
import sources.nixpkgs {
  overlays = [
    # rust nightly overlay
    (import sources.fenix)

    # make the rust toolchain available
    (import ./nix/rust.nix)
  ];
}
