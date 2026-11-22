#!/bin/sh
# Rebuilds results/index.json from the official result files next to it: each file's SHA-256 (what
# the site checks a dropped file against) and a summary for the leaderboard, plus, for every suite
# they ran, a hash of each section's program names, so a file that claims the suite but ran other
# programs shows up as spoofed. Run it after adding or replacing a result. Needs jq and sha256sum.
set -eu
cd "$(dirname "$0")/results"
# decompilers that only run as a web API: their time is the network's, so the board says so
web='["lua.expert", "composite", "luacid"]'
entries=$(mktemp)
suites=$(mktemp)
trap 'rm -f "$entries" "$suites"' EXIT
for f in *.json; do
	[ "$f" = index.json ] && continue
	sum=$(sha256sum "$f" | cut -d' ' -f1)
	jq -c --arg file "$f" --arg sum "$sum" --argjson web "$web" '{
		file: $file, sha256: $sum, decompiler: .decompiler.name, version: .decompiler.version,
		tier: .suite.tier, hash: .suite.hash, score, totals, created, seconds,
		web: (.decompiler.name as $n | $web | index($n) != null)
	}' "$f" >>"$entries"
	sections=$(jq -r '.sections[].id' "$f" | while read -r id; do
		names=$(jq -j --arg id "$id" '[.sections[] | select(.id == $id) | .tests[].name] | join("\n")' "$f" | sha256sum | cut -d' ' -f1)
		count=$(jq --arg id "$id" '[.sections[] | select(.id == $id) | .tests[]] | length' "$f")
		printf '{"%s": {"count": %s, "names": "%s"}}\n' "$id" "$count" "$names"
	done | jq -s 'add')
	jq -c --argjson sections "$sections" '{(.suite.hash): {name: .suite.name, tier: .suite.tier, profiles: (.profiles | length), sections: $sections}}' "$f" >>"$suites"
done
jq -n --slurpfile e "$entries" --slurpfile s "$suites" '{results: ($e | sort_by((.tier != "quick"), -(.score.passed / .score.total), .decompiler)), suites: ($s | add)}' >index.json
echo "indexed $(jq '.results | length' index.json) results in $(pwd)/index.json"
