default:
	just --list

build: clean
	hugo --minify --gc

clean:
	rm -rf public

start: clean
	hugo server --buildDrafts

init:
	@git submodule update --init --recursive

update-submodules:
	@git submodule update --recursive --remote
