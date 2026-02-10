# Coding Swarm — Implementation Plan

**Goal**: Use an autonomous agent swarm to build the core engine, APIs, ingestion systems, and internal tools of a self-driving AI infrastructure platform, while keeping humans at the system boundary.

## 0. First Principles

1.  **Agents build systems, not products.**
2.  **Everything is spec- and test-driven.**
3.  **Humans define constraints, not implementations.**
4.  **The swarm must be able to evolve the system later.**

## 1. Architecture

```text
Human (you)
  ↓ defines specs, constraints, acceptance tests
Swarm Orchestrator
  ↓
Agent Roles (planner / builder / reviewer / integrator)
  ↓
Git Repository (single source of truth)
  ↓
CI + Test Harness (objective reality)
```

## 2. Repo Structure

```text
coding-swarm-engine/
├─ specs/                # Human-authored, stable
│  ├─ engine.md
│  ├─ ingestion.md
│  ├─ contracts.md
│  └─ tools.md
├─ tests/                # Acceptance & invariants
│  ├─ engine/
│  ├─ ingestion/
│  └─ tools/
├─ engine/               # Built by swarm
├─ ingestion/            # Built by swarm
├─ api/                  # Built by swarm
├─ tools/                # Built by swarm
├─ orchestrator/         # Swarm runtime
├─ swarm/                # Agent configs & prompts
├─ ci/
└─ README.md
```

## 3. Usage

1.  **Initialize**: `swarm init` (if not already done).
2.  **Configure**: Edit `swarm.yaml` to define your agents constraints.
3.  **Launch**: `swarm launch` to start the swarm.
4.  **Monitor**: `swarm dashboard` to watch the agents work.

## 4. MVP Scope (v0.1)

-   ✅ **Phase 1**: Core Engine (Scheduler, proper dag, error handling)
-   ✅ **Phase 2**: Ingestion (HTTP, Events, Batch, Schema Inference)
-   ✅ **Phase 3**: Contracts (OpenAPI, API Server)
-   ✅ **Phase 4**: Internal Tools (Replay, Inspection, Load Sim)

