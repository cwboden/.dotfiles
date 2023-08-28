#!/bin/sh

# Replace `last_modified_at` timestamp with current time
# Taken from https://mademistakes.com/notes/adding-last-modified-timestamps-with-git/

git diff --cached --name-status | rg -i "^(A|M).*(_posts).*\.(md)$" | while read a b; do
	cat $b | sed "/---.*/,/---.*/s/^last_modified_at:.*$/last_modified_at: $(date -u "+%Y-%m-%dT%H:%M:%S")/" >tmp
	mv tmp $b
	git add $b
done
