# Testing the decoder with real encodings

Decoding bugs are subtle, so tests use encodings verified with `arm-none-eabi-as`.

The BL backwards test uses the exact bytes the assembler emits for `bl .`, not a value computed by hand.
