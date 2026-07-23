# Flat memory keeps one backing store per region

Regions are named and their data lives in a hash map keyed by name. Lookups are linear because a real map has a dozen regions.

A radix tree would add bug surface for no measurable gain.
