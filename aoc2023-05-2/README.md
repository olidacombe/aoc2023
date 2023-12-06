# Brute force

This one was done badly, I just let the greedy algo run until it returned an answer.

Given more time:

1. `rayon` - slight boost
2. Mapping merge

## Mapping merge

A map followed by a map is basically just one map. So reduce the stack of maps to one, and run backwards from the locations until we hit a seed in our init list.
