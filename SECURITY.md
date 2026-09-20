# Security policy

`mmio-fuzz` is a security tool; it also parses untrusted input (firmware
images, model and testcase files). Please treat both the tool and its parsers
as security-relevant.

## Reporting

Report suspected vulnerabilities privately to the maintainer rather than in a
public issue. Include the firmware or file that triggers the problem, the
command you ran, and the observed behaviour.

## Scope

In scope:

* memory-safety issues in the parsers (`mmio-emu::loader`, model and testcase
  deserialisation);
* denial of service where a small input causes unbounded work.

Out of scope:

* findings the fuzzer reports about a *target* firmware — those are the point
  of the tool;
* crashes in deliberately buggy fixtures.
