{
  outputs =
    { self, nixpkgs, ... }@inputs:
    {
      formatter.x86_64-linux = nixpkgs.legacyPackages.x86_64-linux.nixfmt-rfc-style;
      devShells.x86_64-linux.default = nixpkgs.legacyPackages.x86_64-linux.mkShell {
        packages = with nixpkgs.legacyPackages.x86_64-linux; [
          just
          lua54Packages.lua
          lua-language-server
        ];
        LUA_CPATH = "./target/release/?.so";
        shellHook = ''
          echo "Plunder dev shell"
        '';
      };

    };
}
