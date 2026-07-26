# Decode on five bits, not six

A classic Thumb decoding bug is matching on bits 15:10 when the sixth bit is already an operand.

We select the group with bits 15:11 and then refine. The LSL/LSR/ASR family made this concrete during development.
