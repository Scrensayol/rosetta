#!/bin/sh
# lua.expert (https://lua.expert) as a Rosetta decompiler: POST the bytecode, base64 in JSON.
# Prints the source without the "-- https://lua.expert" header. An HTTP error is a refusal (exit 3),
# except a rate limit (429), which is waited out and asked again: it says nothing about the program.
# The API allows 500 requests a minute: run Rosetta with RAYON_NUM_THREADS=2 or so.
set -eu
req=$(mktemp)
body=$(mktemp)
trap 'rm -f "$req" "$body"' EXIT
base64 -w0 "$1" | jq -Rc '{script: .}' >"$req"
for delay in 2 4 8 16 30 0; do
	code=$(curl -sS -m 25 -o "$body" -w '%{http_code}' -H 'Content-Type: application/json' \
		--data-binary @"$req" https://api.lua.expert/v1/decompile) || {
		echo "unsupported: request failed" >&2
		exit 3
	}
	[ "$code" = 429 ] && [ "$delay" -gt 0 ] || break
	sleep "$delay"
done
if [ "$code" != 200 ]; then
	echo "unsupported: HTTP $code: $(head -c 200 "$body" | tr '\n' ' ')" >&2
	exit 3
fi
sed '1{/^-- https:\/\/lua\.expert/d}' "$body"
