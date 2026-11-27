# LMLM GPU Memory Architecture

> **Status: target architecture / partial prototype.** This document describes the intended design. The repository currently contains a Metal-backed symbolic hash-table prototype; semantic embedding retrieval is not implemented yet.

# **LMLM CORE**

Reasoning · Agents · Model execution

**Memory API**

Store · Retrieve · Update · Delete

**Symbolic memory**

Exact key-value lookup

`123 → value`

`456 → value`

`789 → value`

**Semantic memory**

Embedding similarity

`Vector A`

`Vector B`

`Vector C`

**GPU memory backend**

Metal compute

Parallel kernels

Unified memory

Shared buffers

**Target hardware backend interface (planned)**

Metal

CUDA

Vulkan

CPU

## **Example: how retrieval works**

1. **LMLM receives a query**

   "Find memories related to neural network training."
2. **Semantic memory searches embeddings (planned)**

   A future vector kernel will compute cosine similarities. This kernel is not yet implemented.
3. **Symbolic memory resolves records**

   Retrieved memory IDs point to the associated records.
4. **LMLM receives ranked memories**

   The reasoning engine uses the retrieved information.

This is the intended architecture; the Metal implementation is still a prototype and requires hardware testing.


## Implementation status

### Present in this repository

- `model_training/Dispatcher.py`: Python/Metal dispatcher for a uint32 key-value hash table.
- `model_training/kernels/lmlm_hash.metal`: Metal source for parallel insertion and lookup.
- Shared Metal buffers, linear probing, and GPU dispatch are implemented as a prototype.
- Cross-platform dispatch-contract tests validate host-side buffer-index mappings without requiring Apple hardware.

### Not implemented yet

- Semantic embedding storage and cosine-similarity kernels.
- GPU-side top-k selection.
- Memory deletion, compaction, eviction, and durable persistence.
- A backend-neutral memory runtime interface.
- CUDA, Vulkan, and CPU implementations of this memory API.
- Integration with the Rust extension runtime or LMLM Processor v1 bridge.

Metal execution still requires macOS and a compatible Apple GPU. Hosted Linux CI validates the dispatch contract, not Metal shader compilation or hardware execution.
