# Ordering of plausible read values

The constant value comes first, then the ready value, then observed values, then single bits, all ones and zero.

The order is a priority: keeping already-working paths alive beats exploring, so the safe values sort first. Selection still varies with the input so exploration happens.
