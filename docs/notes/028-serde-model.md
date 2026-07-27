# The model is serde-serialisable

Register and block types derive Serialize and Deserialize.

The model is the main output of inference and the input to reporting, so JSON is the natural interchange. Sets are kept ordered so output is stable across runs.
