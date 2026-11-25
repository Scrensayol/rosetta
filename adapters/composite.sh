#!/bin/sh
# Composite (https://composite.mspaint.cc) as a Rosetta decompiler: POST the bytecode, base64,
# with telemetry off so the server doesn't keep test programs. Prints the source; an error the
# server reports is a refusal (exit 3), except a rate limit (429), which is waited out.
set -eu
req=$(mktemp)
body=$(mktemp)
trap 'rm -f "$req" "$body"' EXIT
base64 -w0 "$1" >"$req"
for delay in 2 4 8 16 30 0; do
	code=$(curl -sS -m 25 -o "$body" -w '%{http_code}' -H 'Content-Type: text/plain' \
		--data-binary @"$req" 'https://composite.mspaint.cc/decompile?encoding=base64&telemetry=false') || {
		echo "unsupported: request failed" >&2
		exit 3
	}
	[ "$code" = 429 ] && [ "$delay" -gt 0 ] || break
	sleep "$delay"
done
if [ "$(jq -r '.ok' "$body" 2>/dev/null)" = true ]; then
	jq -r '.source' "$body"
else
	echo "unsupported: HTTP $code: $(jq -r '.error // empty' "$body" 2>/dev/null | head -c 200 | tr '\n' ' ')" >&2
	exit 3
fi
