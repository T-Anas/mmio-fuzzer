# A minimal privilege model

CONTROL and PRIMASK are stored and can be read and written through MSR/MRS.

There is no Memory Protection Unit, so unprivileged mode has no teeth in the model. It exists so firmware that manipulates CONTROL still runs.
