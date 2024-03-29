{
	description = "noise TUI";

	inputs = {
		nixpkgs.url = "github:NixOS/nixpkgs";
		flake-utils.url = "github:numtide/flake-utils";
	};

	outputs = { self, nixpkgs, flake-utils, }:
		flake-utils.lib.eachDefaultSystem (system: 
			let 
				overlays = [];
				lib = nixpkgs.lib;
				pkgs = import nixpkgs { inherit system overlays; };
				manifest = (lib.importTOML ./Cargo.toml).package;
				rustBuild = (pkgs.rustPlatform.buildRustPackage) {
					pname = manifest.name;
					version = manifest.version;
					nativeBuildInputs = with pkgs; [ pkg-config ];
					buildInputs = with pkgs; [ alsa-lib ];
					cargoLock.lockFile = ./Cargo.lock;
					src = ./.;
				};
			in
				{
					packages = {
						rust = rustBuild;
					};
					defaultPackage = rustBuild;
					devShell = pkgs.mkShell {
						packages = [ pkgs.cava pkgs.cargo pkgs.rustc pkgs.rustfmt pkgs.pkg-config pkgs.alsa-lib ];
						shellHook = ''
						'';
					};
				}
		);
}
