#!/bin/sh
# Fission (https://github.com/SecondNewtonLaw/Fission) as a Rosetta decompiler, through its own
# HTTP server. Start that first: Fission.Server --port 8790 --max-concurrency 8 --max-queued 64
# (FISSION_URL overrides http://127.0.0.1:8790). Default flags, vanilla mode. Prints the source;
# an error the server reports is a refusal (exit 3), and a busy server (503) is asked again.
set -eu
url=${FISSION_URL:-http://127.0.0.1:8790}
req=$(mktemp)
body=$(mktemp)
trap 'rm -f "$req" "$body"' EXIT
base64 -w0 "$1" | jq -Rc '{mode: "vanilla", bytecode: ., timeout: 25}' >"$req"
for _ in 1 2 3 4 5 6 7 8 9 10; do
	code=$(curl -sS -m 28 -o "$body" -w '%{http_code}' -H 'Content-Type: application/json' \
		--data-binary @"$req" "$url/decompile") || {
		echo "unsupported: request failed" >&2
		exit 3
	}
	[ "$code" = 503 ] || break
	sleep 0.2
done
if [ "$(jq -r '.ok' "$body" 2>/dev/null)" = true ]; then
	jq -r '.result.decompilationOutput' "$body"
else
	echo "unsupported: HTTP $code: $(jq -r '.error // .result.resultCode // empty' "$body" 2>/dev/null | head -c 200 | tr '\n' ' ')" >&2
	exit 3
fi
