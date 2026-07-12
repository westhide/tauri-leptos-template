{
  pkgs ? import <nixpkgs> {
    config = {
      allowUnfree = true;
      android_sdk.accept_license = true;
    };
  },
  ANDROID_HOME ? "$HOME/Android/Sdk",
}:

# let
#   AndroidPkgs = pkgs.androidenv.composeAndroidPackages {
#     includeNDK = true;
#     includeEmulator = true;
#     includeSystemImages = true;

#     buildToolsVersions = [
#       "latest"
#     ];
#     platformVersions = [ "36" ];
#   };
# in
with pkgs;
mkShell {
  # shellHook = ''
  #   export ANDROID_SDK_ROOT="${AndroidPkgs.androidsdk}/libexec/android-sdk";
  #   export ANDROID_NDK_ROOT="$ANDROID_SDK_ROOT/ndk-bundle";

  #   export ANDROID_HOME="$ANDROID_SDK_ROOT";
  #   export NDK_HOME="$ANDROID_NDK_ROOT";
  #   export WEBKIT_DISABLE_DMABUF_RENDERER=1;
  # '';

  nativeBuildInputs = with pkgs; [
    pkg-config
    rustPlatform.bindgenHook
    wrapGAppsHook4
  ];

  buildInputs = with pkgs; [
    webkitgtk_4_1
  ];
}
