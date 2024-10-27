let
  pkgs = import <nixpkgs> { };
in
pkgs.rustPlatform.buildRustPackage {
  name = "boycottcarti";
  src = ./.;
  cargoLock.lockFile = ./Cargo.lock;
  buildInputs = [ pkgs.dbus ];
  nativeBuildInputs = [ pkgs.pkg-config ];
  meta = {
    description = "Stop Carti from playing on your computer";
    homepage = "https://github.com/Jack5079/boycottcarti";
    license = pkgs.lib.licenses.mpl20;
    mainProgram = "boycottcarti";
  };
}
