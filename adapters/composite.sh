#!/bin/sh
# Composite (https://composite.mspaint.cc) as a Rosetta decompiler: POST the bytecode, base64,
# with telemetry off so the server doesn't keep test programs. Prints the source; an error the
# server reports is a refusal (exit 3).
set -eu
body=$(mktemp)
trap 'rm -f "$body"' EXIT
code=$(base64 -w0 "$1" | curl -sS -m 25 -o "$body" -w '%{http_code}' -H 'Content-Type: text/plain' \
	--data-binary @- 'https://composite.mspaint.cc/decompile?encoding=base64&telemetry=false') || {
	echo "unsupported: request failed" >&2
	exit 3
}
if [ "$(jq -r '.ok' "$body" 2>/dev/null)" = true ]; then
	jq -r '.source' "$body"
else
	echo "unsupported: HTTP $code: $(jq -r '.error // empty' "$body" 2>/dev/null | head -c 200 | tr '\n' ' ')" >&2
	exit 3
fi
