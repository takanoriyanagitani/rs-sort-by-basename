#!/bin/sh

which wazero | fgrep -q wazero || exec sh -c 'echo wazero missing.; exit 1'

printf \
	'%s\n' \
	/path/to/cargo \
	/0000/to/clippy \
	/1000/to/build \
	/2000/to/fmt \
	/3000/to/doc |
	wazero \
		run ./rs-sort-by-basename.wasm
