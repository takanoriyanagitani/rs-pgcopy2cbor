#!/bin/sh

input="./sample.pgcopy"
output="./out.cbor"

bytecode="./rs-pgcopy2cbor.wasm"
native="./rs-pgcopy2cbor"

cat "${input}" |
	\time -l \
	wazero run \
	"${bytecode}" |
	dd \
		if=/dev/stdin \
		of="${output}" \
		bs=1048576 \
		conv=fsync \
		status=progress
