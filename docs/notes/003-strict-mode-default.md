# Strict memory is on by default

An access to an address nothing maps is an error, not a silent zero.

That is what turns a firmware bug into an `UnmappedAccess` finding. Fuzzing can flip strict off, but the engine keeps it on so real faults surface.
