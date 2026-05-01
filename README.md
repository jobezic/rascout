# Rascout

A high-performance, Rust-based edge agent for distributed network discovery and incremental device intelligence.

## Overview

Rascout is a lightweight, asynchronous system that scans networks, tracks device state over time, and emits incremental (diff-based) updates to a central ingestion service.

Unlike naive scanners that repeatedly send full snapshots, Sentinel Agent is designed for efficiency, resilience, and scalability:

- Detects changes in device state
- Sends only meaningful updates
- Handles backpressure and transient failures
- Operates safely under constrained resources