# Phase 2: Ingestion Engines Specification
**Goal**: Develop a universal ingestion layer that abstracts data intake from various sources, providing normalized pipelines for the core engine to process. This enables seamless data flow into the self-driving infrastructure without source-specific logic.

## 1. Components
### 1.1 HTTP Listener
- **Responsibility**: Exposes endpoints for receiving data via HTTP (e.g., POST for events).
- **Invariants**:
  - Supports webhook-style ingestion with validation (e.g., signatures).
  - Handles rate limiting to prevent overload (e.g., 100 reqs/sec).
  - Asynchronous processing to avoid blocking.

### 1.2 Event Stream Abstraction
- **Responsibility**: Manages streaming data inputs, buffering and forwarding to the engine.
- **Invariants**:
  - Supports multiple formats (JSON, protobuf, etc.).
  - Ensures ordering for sequential events.
  - Backpressure handling to pause ingestion on high load.

### 1.3 Batch Ingestion
- **Responsibility**: Processes bulk data uploads efficiently.
- **Invariants**:
  - Atomic commits (all or nothing for batches).
  - Chunking for large payloads (>1MB split into 100KB chunks).
  - Deduplication based on unique identifiers.

### 1.4 Schema Inference
- **Responsibility**: Automatically detects and validates data schemas.
- **Invariants**:
  - Infers types (string, int, etc.) from samples.
  - Rejects invalid data early with error feedback.
  - Supports schema evolution (e.g., adding fields).

### 1.5 Normalization Pipeline
- **Responsibility**: Transforms raw data into a standardized format for the engine.
- **Invariants**:
  - Configurable transformations (e.g., via YAML rules).
  - Preserves provenance (source, timestamp).
  - Idempotent processing.

## 2. Constraints
- **Language**: Python 3.10+; use `asyncio` for async ops, `jsonschema` if needed (standard lib preferred).
- **Testing**: 100% coverage; simulate inputs with mock servers.
- **No Specific Integrations**: Build generic interfaces only (adapters added later).
- **Performance**: Process 10MB/sec; low memory footprint.
- **Security**: Input sanitization to prevent injection attacks.
