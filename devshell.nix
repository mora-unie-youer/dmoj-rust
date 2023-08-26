{ pkgs, ... }:

pkgs.devShell.mkShell {
  name = "dmoj";
  packages = with pkgs; [
    # Toolchain required for C + Rust binaries building
    binutils
    gcc

    # Rust 1.71.1 toolchain as DM::OJ supports only this version
    bacon
    cargo-flamegraph
    (rust-bin.stable."1.71.1".default.override {
      # Extensions which ease your development process
      extensions = [ "rust-analyzer" "rust-src" ];
    })
  ];
}
