{
  pkgs ? import <nixpkgs> {
    config = {
      allowUnfree = true;
    };
  },
}:

with pkgs;
mkShell {
  shellHook = ''
    export WEBKIT_DISABLE_DMABUF_RENDERER=1;
  '';

  nativeBuildInputs = with pkgs; [
    pkg-config
    rustPlatform.bindgenHook
    wrapGAppsHook4
  ];

  buildInputs = with pkgs; [
    webkitgtk_4_1
  ];
}
