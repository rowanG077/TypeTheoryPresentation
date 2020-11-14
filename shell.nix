with import <nixpkgs> { };

clangStdenv.mkDerivation {
  name = "type-theory";

  buildInputs = [
    # Rust shizzle
    cargo
    rustfmt
    rls
    rustc
    rustup

    # C shizzle
    gcc
    clang_11

    # Presentation shizzle
    texlive.combined.scheme-full
  ];
}
