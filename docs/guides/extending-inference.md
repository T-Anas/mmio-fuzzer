# Extending inference

Add a detector in `mmio-infer` that takes the per-address event stream.
Record the new property on `RegisterModel` and surface it in `summary`.
Test it against a synthetic trace with `AccessLog::from_vec`.
