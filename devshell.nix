{ pkgs, ... }:

pkgs.devShell.mkShell {
  name = "dmoj";
  packages = with pkgs; [
    # Toolchain required for C + Rust binaries building
    binutils
    gcc

    # Rust 1.70.0 toolchain as DM::OJ supports only this version
    bacon
    cargo-flamegraph
    (rust-bin.stable."1.70.0".default.override {
      # Extensions which ease your development process
      extensions = [ "rust-analyzer" "rust-src" ];
    })
  ];
}
