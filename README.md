# Description

HackerRank challenge; find the minimum loss between two numbers where the first
number occurs earlier in the array.

# Lessons

- Use of BTreeSet
- stdin piped to app: `cargo run --release < input.txt`
- check for overflow
- include a couple of heuristics

# Note

The use of both the BTreeSet and a tuple that records the values position in the
array may be redundant.
