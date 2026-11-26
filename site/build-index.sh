#!/bin/sh
# Rebuilds results/index.json from the official result files next to it: each file's SHA-256 (what
# the site checks a dropped file against) and a summary for the leaderboard, plus, for every suite
# they ran, a hash of each section's program names, so a file that claims the suite but ran other
# programs shows up as spoofed. Hard-tier results also get their score at each transform level.
# Run it after adding or replacing a result. Needs jq and sha256sum.
# Results of an older suite live in a directory named after it (results/rosetta-1/): they stay
# verified and the leaderboard can still show them, as archived.
set -eu
cd "$(dirname "$0")/results"
# decompilers that only run as a web API: their time is the network's, so the board says so
web='["lua.expert", "composite", "luacid", "luarust"]'
entries=$(mktemp)
suites=$(mktemp)
trap 'rm -f "$entries" "$suites"' EXIT
for f in *.json */*.json; do
	[ -f "$f" ] || continue
	[ "$f" = index.json ] && continue
	case $f in */*) archived=true ;; *) archived=false ;; esac
	sum=$(sha256sum "$f" | cut -d' ' -f1)
	jq -c --arg file "$f" --arg sum "$sum" --argjson web "$web" --argjson archived "$archived" '{
		file: $file, sha256: $sum, decompiler: .decompiler.name, version: .decompiler.version,
		tier: .suite.tier, hash: .suite.hash, score, totals, created, seconds,
		web: (.decompiler.name as $n | $web | index($n) != null), archived: $archived,
		levels: ([.profiles[] | (capture("\\+x(?<l>[0-9])") // {l: null}) | .l] as $lv
			| if ($lv | map(select(. != null)) | length) == 0 then null else
				reduce (.sections[].tests[].results | split("") | to_entries[]) as $e ({};
					$lv[$e.key] as $l
					| if $l == null or $e.value == "u" then . else .[$l].total += 1 | (if $e.value == "p" then .[$l].passed += 1 else . end) end)
				| with_entries(.value.passed //= 0)
			end)
	}' "$f" >>"$entries"
	sections=$(jq -r '.sections[].id' "$f" | while read -r id; do
		names=$(jq -j --arg id "$id" '[.sections[] | select(.id == $id) | .tests[].name] | join("\n")' "$f" | sha256sum | cut -d' ' -f1)
		count=$(jq --arg id "$id" '[.sections[] | select(.id == $id) | .tests[]] | length' "$f")
		printf '{"%s": {"count": %s, "names": "%s"}}\n' "$id" "$count" "$names"
	done | jq -s 'add')
	jq -c --argjson sections "$sections" '{(.suite.hash): {name: .suite.name, tier: .suite.tier, profiles: (.profiles | length), sections: $sections}}' "$f" >>"$suites"
done
jq -n --slurpfile e "$entries" --slurpfile s "$suites" '{results: ($e | sort_by(.archived, (.tier != "quick"), -(.score.passed / .score.total), .decompiler)), suites: ($s | add)}' >index.json
echo "indexed $(jq '.results | length' index.json) results in $(pwd)/index.json"
