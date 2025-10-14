# Telemetry & Structured Logging Notes

_Status: exploratory design. Not yet implemented._

## Motivation
- Large files (multi-GB) can take minutes to hash; users frequently assume the app has frozen.
- Support tickets arrive without diagnostics—structured logs would help reproduce I/O failures, permission issues, or short reads.
- Optional telemetry would allow enterprise deployments to forward anonymised metrics (duration, algorithm, error codes) to monitoring systems.

## Proposed Scope
- Emit structured events (JSON) for start/completion, bytes processed, algorithm, warnings/errors.
- Add CLI/GUI progress reporting built on the same event stream.
- Provide opt-in sinks: local rotating log file, or HTTP endpoint specified via flags/env vars.
- Keep defaults privacy-focused: telemetry disabled unless explicitly configured.

## Open Questions
- Data minimisation (avoid leaking filenames/digests unless user consents).
- Batching/backpressure when sending to remote endpoints.
- Integration with the upcoming plugin API so third-party algorithms can reuse the logging facade.

Tracked under Phase 5 stretch goals.
