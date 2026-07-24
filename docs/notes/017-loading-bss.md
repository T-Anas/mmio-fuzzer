# BSS stays zeroed

Only file-backed bytes are copied when applying an image.

`.bss` is zero by definition and RAM regions start zeroed, so there is nothing to do. Explicitly zeroing would only risk clobbering initialised data.
