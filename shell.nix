{ pkgs ? import (fetchTarball https://github.com/NixOS/nixpkgs/archive/refs/tags/21.05.tar.gz) {} }:
let
    frameworks = pkgs.darwin.apple_sdk.frameworks;
in pkgs.mkShell {
    buildInputs = [ pkgs.rustc
                    pkgs.cargo
                    pkgs.libiconv
                    frameworks.Security
                    frameworks.CoreFoundation
                    frameworks.CoreServices
                  ];

    shellHook = ''
        export NIX_LDFLAGS="-F${frameworks.CoreFoundation}/Library/Frameworks -framework CoreFoundation $NIX_LDFLAGS";
    '';
}
