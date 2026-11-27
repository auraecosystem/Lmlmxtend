#include <metal_stdlib>
using namespace metal;

constant uint EMPTY_KEY = 0xFFFFFFFFu;
constant uint LOCKED_KEY = 0xFFFFFFFEu;

inline uint hash_key(uint key, uint capacity) {
    return (key * 2654435761u) & (capacity - 1u);
}

// Insert one unique key per input thread. The host must not issue overlapping
// insert dispatches concurrently. A later dispatch observes completed writes.
kernel void hash_insert(
    device const uint* input_keys       [[buffer(0)]],
    device const uint* input_values     [[buffer(1)]],
    device atomic_uint* table_keys      [[buffer(2)]],
    device uint* table_values           [[buffer(3)]],
    constant uint& capacity             [[buffer(4)]],
    constant uint& count                [[buffer(5)]],
    device uint* results                [[buffer(6)]],
    uint tid [[thread_position_in_grid]]
) {
    if (tid >= count) return;

    uint key = input_keys[tid];
    uint value = input_values[tid];

    if (key >= LOCKED_KEY) {
        results[tid] = 0u;
        return;
    }

    uint start = hash_key(key, capacity);

    for (uint probe = 0; probe < capacity; ++probe) {
        uint slot = (start + probe) & (capacity - 1u);
        uint expected = EMPTY_KEY;

        bool claimed = atomic_compare_exchange_weak_explicit(
            &table_keys[slot],
            &expected,
            LOCKED_KEY,
            memory_order_relaxed,
            memory_order_relaxed
        );

        if (claimed) {
            table_values[slot] = value;
            atomic_store_explicit(
                &table_keys[slot],
                key,
                memory_order_release
            );
            results[tid] = 1u;
            return;
        }

        if (expected == key) {
            table_values[slot] = value;
            results[tid] = 1u;
            return;
        }
    }

    results[tid] = 0u;
}

// Lookup is dispatched after insertion completion. Each thread resolves one key.
kernel void hash_lookup(
    device const uint* query_keys         [[buffer(0)]],
    device const atomic_uint* table_keys  [[buffer(1)]],
    device const uint* table_values       [[buffer(2)]],
    constant uint& capacity               [[buffer(3)]],
    constant uint& count                  [[buffer(4)]],
    device uint* output_values            [[buffer(5)]],
    device uint* found                    [[buffer(6)]],
    uint tid [[thread_position_in_grid]]
) {
    if (tid >= count) return;

    uint key = query_keys[tid];
    if (key >= LOCKED_KEY) {
        found[tid] = 0u;
        output_values[tid] = 0u;
        return;
    }

    uint start = hash_key(key, capacity);

    for (uint probe = 0; probe < capacity; ++probe) {
        uint slot = (start + probe) & (capacity - 1u);
        uint stored_key = atomic_load_explicit(
            &table_keys[slot],
            memory_order_acquire
        );

        if (stored_key == EMPTY_KEY) {
            found[tid] = 0u;
            output_values[tid] = 0u;
            return;
        }

        if (stored_key == key) {
            output_values[tid] = table_values[slot];
            found[tid] = 1u;
            return;
        }
    }

    found[tid] = 0u;
    output_values[tid] = 0u;
}
