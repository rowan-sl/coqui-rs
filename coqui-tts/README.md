# Coqui-TTS

rust bindings to the [coqui-ai TTS](https://github.com/coqui-ai/TTS) python lib, using pyo3.

## Note about features

This crate does not come anywhere close to the available feature set of the underlying library,
and is meant only to provide a way to synthesize speech in rust (that works well, and does not use a cloud service)

This is not one of my high-effort projects, and if you would like to see more features implemented,
feel free to ask in the [repo](https://github.com/rowan-sl/coqui-rs/tree/main)

## Python dependanices

this depends on the `TTS` package, (tested to work with up to v0.12.0). it is recommended to install it in a python virtual environment such as `venv`
