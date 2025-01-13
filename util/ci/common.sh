#!/bin/bash -e

# Linux build only
install_linux_deps() {
	local pkgs=(
		cmake gettext
		libpng-dev libjpeg-dev libgl1-mesa-dev libsdl2-dev libfreetype-dev
		libsqlite3-dev libogg-dev libgmp-dev libvorbis-dev
		libopenal-dev libpq-dev libcurl4-openssl-dev libzstd-dev
		libssl-dev
	)

	sudo apt-get update
	sudo apt-get install -y --no-install-recommends "${pkgs[@]}" "$@"

}
