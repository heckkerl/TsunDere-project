#!/bin/zsh

echo "Building..."
bun install

bun build --compile --minify --target=bun-linux-x64 ./index.ts --outfile dist/tsundere-linux
echo "Ok Linux"

bun build --compile --minify --target=bun-windows-x64 ./index.ts --outfile dist/tsundere-windows.exe
echo "Ok Windows"

bun build --compile --minify --target=bun-darwin-arm64 ./index.ts --outfile dist/tsundere-darwin
echo "Hey Siri"

echo "Ok Computer"
