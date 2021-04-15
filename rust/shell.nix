let
  pkgs = import ./.;
in
with pkgs;
mkShell {
  name = "exercism";
  buildInputs = [
    cargo-bloat
    cargo-edit
    cargo-release
    cargo-udeps
    curl
    exercism
    gh
    jq
    niv
    openssl
    pkg-config
    rustToolchain
    rustc
    yj
    (python3.withPackages (p: with p; [
      python-language-server
      pyls-black
      pyls-mypy
      pyls-isort
      flake8
      ipdb
      ipython
    ]))
  ];
}
