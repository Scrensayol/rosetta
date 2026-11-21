#!/bin/sh
# lua.expert (https://lua.expert) as a Rosetta decompiler: POST the bytecode, base64 in JSON.
# Prints the source without the "-- https://lua.expert" header. An HTTP error is a refusal (exit 3).
# The API allows 500 requests a minute: run Rosetta with RAYON_NUM_THREADS=2 or so.
set -eu
body=$(mktemp)
trap 'rm -f "$body"' EXIT
code=$(base64 -w0 "$1" | jq -Rc '{script: .}' | curl -sS -m 25 -o "$body" -w '%{http_code}' \
	-H 'Content-Type: application/json' --data-binary @- https://api.lua.expert/v1/decompile) || {
	echo "unsupported: request failed" >&2
	exit 3
}
if [ "$code" != 200 ]; then
	echo "unsupported: HTTP $code: $(head -c 200 "$body" | tr '\n' ' ')" >&2
	exit 3
fi
sed '1{/^-- https:\/\/lua\.expert/d}' "$body"
