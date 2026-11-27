# LMLM GPU Memory Architecture

> **Status: target architecture / partial prototype.** This document describes the intended design. The current repository contains a Metal hash-table prototype; it does not yet contain a semantic embedding store or a complete multi-backend memory subsystem.

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

   A future vector kernel will compute cosine similarities. This kernel is not yet implemented in this repository.
3. **Symbolic memory resolves records**

   Retrieved memory IDs point to the associated records.
4. **LMLM receives ranked memories**

   The reasoning engine uses the retrieved information.

This is the intended architecture; the Metal implementation is still a prototype and requires hardware testing.


## Implementation status

### Present in this repository

- `model_training/Dispatcher.py`: Python/Metal dispatcher for a uint32 key-value hash table.
- `src/compute_kernel.meta`: Metal source containing parallel hash insertion and lookup kernels.
- Shared Metal buffers, linear probing, and GPU dispatch are implemented as a prototype.

### Not implemented yet

- Semantic embedding storage and cosine-similarity kernels.
- GPU-side top-k selection.
- Memory deletion, compaction, eviction, and durable persistence.
- A backend-neutral runtime interface.
- CUDA, Vulkan, and CPU implementations of this memory API.
- Integration with the Rust extension runtime or the LMLM Processor v1 bridge.

Metal execution requires macOS and a compatible Apple GPU. The current hosted Rust CI does not compile or execute Metal shaders. Hardware validation remains outstanding.
