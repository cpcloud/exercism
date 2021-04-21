let
  pkgs = import ./.;
in
with pkgs;
mkShell {
  name = "exercism-cpp";
  buildInputs = [
    cmake
    curl
    exercism
    gh
    jq
    niv
    clang_11
    llvm_11
    openssl
    pkg-config
    yj
  ];
}
