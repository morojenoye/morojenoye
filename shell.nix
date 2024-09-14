{ pkgsRV ? import <nixpkgs> { crossSystem = "riscv64-linux"; }
, pkgsHS ? import <nixpkgs> { } }:

pkgsHS.mkShell {
  packages = [
    pkgsRV.buildPackages.binutils
    pkgsRV.buildPackages.gcc
    pkgsRV.buildPackages.gdb
  ];
}
