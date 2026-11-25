#!/bin/sh
# Luacid (https://luacid.dev) as a Rosetta decompiler: POST the raw bytecode to the keyless API with
# default options. Prints the source without the "-- [[ luacid.dev ]]" header line. An HTTP error is
# a refusal (exit 3), except a rate limit (429), which is waited out and asked again. The keyless
# tier allows 60 requests a minute per IP, so requests are spaced 1.05 s apart through a lock
# file: run Rosetta with RAYON_NUM_THREADS=1 so none waits long.
# LUACID_KEY, if set, is sent as a Bearer key.
set -eu
input=$1
body=$(mktemp)
trap 'rm -f "$body"' EXIT
stamp=${TMPDIR:-/tmp}/rosetta-luacid.stamp
auth=
if [ -n "${LUACID_KEY:-}" ]; then auth="Authorization: Bearer $LUACID_KEY"; fi
for delay in 5 10 20 40 0; do
	# one request at a time across every copy of this script, 1.05 s apart
	exec 9>"$stamp.lock"
	flock 9
	now=$(date +%s%N)
	last=$(cat "$stamp" 2>/dev/null || echo 0)
	wait=$(( (last + 1050000000 - now) / 1000000 ))
	if [ "$wait" -gt 0 ]; then sleep "$(printf '%d.%03d' $((wait / 1000)) $((wait % 1000)))"; fi
	date +%s%N >"$stamp"
	flock -u 9
	code=$(curl -sS -m 25 -o "$body" -w '%{http_code}' -H 'Content-Type: application/octet-stream' \
		${auth:+-H "$auth"} --data-binary @"$input" https://api.luacid.dev/decompile) || {
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
sed '1{/^-- \[\[ luacid\.dev \]\]/d}' "$body"
