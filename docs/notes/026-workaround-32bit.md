# 32-bit encodings we support

ARMv6-M defines only BL, the barriers and MRS/MSR as 32-bit.

Anything else decodes to `Unsupported` and becomes an `InvalidOpcode` finding, which is honest: we would rather report a gap than execute the wrong thing.
