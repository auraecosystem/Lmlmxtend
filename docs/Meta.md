Here is the complete LMLM GPU memory architecture, showing how symbolic memory and semantic memory work together.

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

**Hardware backend interface**

Metal

CUDA

Vulkan

CPU

## **Example: how retrieval works**

1. **LMLM receives a query**

   "Find memories related to neural network training."
2. **Semantic memory searches embeddings**

   Metal computes cosine similarities.
3. **Symbolic memory resolves records**

   Retrieved memory IDs point to the associated records.
4. **LMLM receives ranked memories**

   The reasoning engine uses the retrieved information.

This is the intended architecture; the Metal implementation is still a prototype and requires hardware testing.
