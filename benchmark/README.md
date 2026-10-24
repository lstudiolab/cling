# Benchmarks

Performance work begins after a stable end-to-end pipeline exists. Every benchmark must state compiler revision, build mode, host architecture, input corpus, warm-up policy, and measurement command. Keep compile-time measurements separate from generated-program runtime measurements.

Do not publish performance claims without a reproducible baseline. The current C++ bootstrap supports source management and lexical analysis only, so benchmarks are not yet part of CI.
