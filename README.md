# Rascout

A high-performance, Rust-based edge agent for distributed network discovery and incremental device intelligence.

The project explores the engineering challenges behind distributed asset discovery systems:

- asynchronous network discovery
- bounded concurrency
- event-driven processing
- state reconciliation
- incremental updates
- backpressure
- resilient event delivery
- observability
- eventual consistency

The primary goal is not to build another network scanner. It is to explore how a network-edge agent can efficiently discover and continuously report changing infrastructure to a distributed backend.

## Overview

Rascout is a lightweight, asynchronous system that scans networks, tracks device state over time, and emits incremental (diff-based) updates to a central ingestion service.

Unlike naive scanners that repeatedly send full snapshots, Sentinel Agent is designed for efficiency, resilience, and scalability:

- Detects changes in device state
- Sends only meaningful updates
- Handles backpressure and transient failures
- Operates safely under constrained resources

Architecture
  
```
 ┌─────────────────┐
 │                 │
 │  Rascout Agent  │
 │                 │
 │  ┌───────────┐  │
 │  │ Scheduler │  │
 │  └─────┬─────┘  │
 │        │        │
 │  ┌─────▼─────┐  │
 │  │  Scanner  │  │
 │  └─────┬─────┘  │
 │        │        │
 │  ┌─────▼─────┐  │
 │  │   State   │  │
 │  │   + Diff  │  │
 │  └─────┬─────┘  │
 │        │        │
 │  ┌─────▼─────┐  │
 │  │   Sender  │  │
 │  └─────┬─────┘  │
 └────────┼────────┘
          │
          │ HTTP / Events
          ▼
 ┌────────────────────┐
 │                    │
 │   Ingestion API    │
 │                    │
 └─────────┬──────────┘
           │
           ▼
 ┌────────────────────┐
 │                    │
 │ Processing Pipeline│
 │                    │
 └─────────┬──────────┘
           │
           ▼
 ┌────────────────────┐
 │     PostgreSQL     │
 │                    │
 └────────────────────┘
 ```

 ### Rascout Agent

The agent is the edge component of Rascout. Its responsibility is:
- observe the network,
- maintain local state,
- detect changes,
- reliably report those changes.

#### Architecture

```
Scheduler
    │
    ▼
Scanner
    │
    ▼
State / Diff Engine
    │
    ▼
Event Sender
```

#### Core Design Principles

##### 1. Bounded Concurrency

Network discovery can easily create thousands of concurrent operations. The agent therefore explicitly limits concurrency:

```
The agent therefore explicitly limits concurrency:
                ┌─────────┐
target ────────►│ Worker  │
target ────────►│ Worker  │
target ────────►│ Worker  │
target ────────►│ Worker  │
                └─────────┘
                     ▲
                     │
                Semaphore
```

The goal is to make resource consumption predictable rather than allowing an unbounded number of network operations.

##### 2. Backpressure

The internal pipeline uses bounded channels.

```
Scanner
   │
   │ bounded channel
   ▼
Diff Engine
   │
   │ bounded channel
   ▼
Sender
```

If downstream processing becomes slower than discovery, the queue grows only to a defined limit.
This forces the system to make an explicit decision about pressure rather than silently consuming more memory.

##### 3. Incremental reporting

The agent maintains a local representation of previously observed state and it only sends the changes in the status to the backend. This reduces unnecessary network traffic and downstream processing.

##### 4. Resilient Delivery

The network between the edge and backend cannot be assumed to be reliable.

The sender therefore supports:

- batching
- retries
- exponential backoff
- request timeouts

The intended delivery model is at-least-once, with idempotency handled by the backend.

This is an intentional trade-off: **losing an event is generally worse than delivering the same event twice.**

