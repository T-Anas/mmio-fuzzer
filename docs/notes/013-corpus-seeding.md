# Seeding the corpus with the empty input

The first execution of a run is done with an empty input so behaviour is deterministic and comparable.

The empty input is added to the corpus so later mutations always have a parent. Corpus entries are deduplicated by coverage digest.
