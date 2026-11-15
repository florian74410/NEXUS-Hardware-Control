# Protocol v0 draft
Language-neutral UTF-8 JSON Lines over serial; bounded frame length (provisional 4096 bytes), version negotiation, sequence, session identifier, heartbeat and per-metric validity. HELLO, INVENTORY, CAPABILITIES, TELEMETRY, HEARTBEAT. No hardware writes in phase 1. Reconnect clears old live values; unsupported metrics are null. IPC shares canonical metric definitions.
