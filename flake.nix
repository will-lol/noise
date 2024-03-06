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
					buildInpus = with pkgs; [ pkg-config ];
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
						packages = [ pkgs.cargo pkgs.rustc pkgs.rustfmt rustBuild ];
						shellHook = ''
						'';
					};
				}
		);
}
