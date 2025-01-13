#!/bin/bash -e

# Linux build only
install_linux_deps() {
	local pkgs=(
		cmake gettext postgresql
		libpng-dev libjpeg-dev libgl1-mesa-dev libsdl2-dev libfreetype-dev
		libsqlite3-dev libhiredis-dev libogg-dev libgmp-dev libvorbis-dev
		libopenal-dev libpq-dev libleveldb-dev libcurl4-openssl-dev libzstd-dev
		libssl-dev
	)

	sudo apt-get update
	sudo apt-get install -y --no-install-recommends "${pkgs[@]}" "$@"

	# set up Postgres for unit tests
	if [ -n "$MINETEST_POSTGRESQL_CONNECT_STRING" ]; then
		sudo systemctl start postgresql.service
		sudo -u postgres psql <<<"
			CREATE USER minetest WITH PASSWORD 'minetest';
			CREATE DATABASE minetest;
			\c minetest
			GRANT ALL ON SCHEMA public TO minetest;
		"
	fi
}
