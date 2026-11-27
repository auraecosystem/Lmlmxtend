"""Metal-backed symbolic key/value memory for Apple platforms.

The Metal backend is optional and is not imported by the Rust workspace.
"""

from pathlib import Path
import ctypes

import Metal
import numpy as np


EMPTY_KEY = 0xFFFFFFFF
LOCKED_KEY = 0xFFFFFFFE
SHARED = Metal.MTLResourceStorageModeShared

DEFAULT_SHADER = (
    Path(__file__).resolve().parent / "kernels" / "lmlm_hash.metal"
)


def check_error(result, error, operation):
    if result is None:
        message = str(error) if error else "Unknown Metal error"
        raise RuntimeError(f"{operation} failed: {message}")
    return result


class MetalHashStore:
    """A fixed-capacity, linear-probing hash table backed by Metal buffers.

    Insertion batches must contain unique keys. Calls are synchronous: each
    dispatch waits for completion before returning. Concurrent calls from
    multiple host threads and concurrent duplicate-key updates are not
    supported by this prototype.
    """

    def __init__(self, capacity=1024 * 1024, shader_path=None):
        if capacity <= 0 or capacity & (capacity - 1):
            raise ValueError("Capacity must be a positive power of two")

        if capacity > 0xFFFFFFFF:
            raise ValueError("Capacity must fit in a uint32")

        self.capacity = capacity
        self.device = Metal.MTLCreateSystemDefaultDevice()

        if self.device is None:
            raise RuntimeError("Metal device unavailable")

        self.queue = self.device.newCommandQueue()
        if self.queue is None:
            raise RuntimeError("Could not create Metal command queue")

        source_path = Path(shader_path) if shader_path else DEFAULT_SHADER
        source = source_path.read_text(encoding="utf-8")

        library, error = self.device.newLibraryWithSource_options_error_(
            source, None, None
        )
        self.library = check_error(library, error, "Metal shader compilation")

        self.insert_pipeline = self._pipeline("hash_insert")
        self.lookup_pipeline = self._pipeline("hash_lookup")

        self.table_keys = self._buffer(
            capacity * np.dtype(np.uint32).itemsize,
            initialize=b"\xff" * (capacity * 4),
        )
        self.table_values = self._buffer(capacity * 4)

    def _pipeline(self, function_name):
        function = self.library.newFunctionWithName_(function_name)
        if function is None:
            raise RuntimeError(f"Missing Metal function: {function_name}")

        pipeline, error = (
            self.device.newComputePipelineStateWithFunction_error_(
                function, None
            )
        )
        return check_error(pipeline, error, function_name)

    def _buffer(self, size, initialize=None):
        if initialize is None:
            buffer = self.device.newBufferWithLength_options_(size, SHARED)
        else:
            source = ctypes.create_string_buffer(initialize)
            buffer = self.device.newBufferWithBytes_length_options_(
                ctypes.addressof(source), size, SHARED
            )

        if buffer is None:
            raise MemoryError(f"Metal buffer allocation failed: {size} bytes")
        return buffer

    def _upload(self, array):
        array = np.ascontiguousarray(array, dtype=np.uint32)
        buffer = self.device.newBufferWithBytes_length_options_(
            array.ctypes.data, array.nbytes, SHARED
        )
        if buffer is None:
            raise MemoryError("Metal input buffer allocation failed")
        return buffer

    @staticmethod
    def _read(buffer, count):
        raw = ctypes.string_at(buffer.contents(), count * 4)
        return np.frombuffer(raw, dtype=np.uint32).copy()

    def _dispatch(
        self,
        pipeline,
        indexed_buffers,
        capacity,
        count,
        capacity_index,
        count_index,
    ):
        if count <= 0:
            return

        command_buffer = self.queue.commandBuffer()
        if command_buffer is None:
            raise RuntimeError("Could not create Metal command buffer")

        encoder = command_buffer.computeCommandEncoder()
        if encoder is None:
            raise RuntimeError("Could not create Metal compute encoder")

        encoder.setComputePipelineState_(pipeline)

        # Bind resources at the exact [[buffer(n)]] indices in the shader.
        for index, buffer in indexed_buffers:
            encoder.setBuffer_offset_atIndex_(buffer, 0, index)

        encoder.setBytes_length_atIndex_(
            np.uint32(capacity).tobytes(), 4, capacity_index
        )
        encoder.setBytes_length_atIndex_(
            np.uint32(count).tobytes(), 4, count_index
        )

        width = pipeline.threadExecutionWidth()
        if width <= 0:
            encoder.endEncoding()
            raise RuntimeError("Metal returned an invalid thread execution width")

        groups = (count + width - 1) // width
        encoder.dispatchThreadgroups_threadsPerThreadgroup_(
            Metal.MTLSizeMake(groups, 1, 1),
            Metal.MTLSizeMake(width, 1, 1),
        )

        encoder.endEncoding()
        command_buffer.commit()
        command_buffer.waitUntilCompleted()

        if command_buffer.status() == Metal.MTLCommandBufferStatusError:
            raise RuntimeError(
                f"Metal execution failed: {command_buffer.error()}"
            )

    def insert(self, keys, values):
        keys = np.asarray(keys, dtype=np.uint32).reshape(-1)
        values = np.asarray(values, dtype=np.uint32).reshape(-1)

        if len(keys) != len(values):
            raise ValueError("Keys and values must have equal lengths")
        if len(np.unique(keys)) != len(keys):
            raise ValueError("Keys must be unique within an insertion batch")
        if np.any(keys >= LOCKED_KEY):
            raise ValueError("0xFFFFFFFE and 0xFFFFFFFF are reserved keys")
        if len(keys) == 0:
            return np.empty(0, dtype=np.uint32)

        input_keys = self._upload(keys)
        input_values = self._upload(values)
        results = self._buffer(len(keys) * 4)

        self._dispatch(
            self.insert_pipeline,
            [
                (0, input_keys),
                (1, input_values),
                (2, self.table_keys),
                (3, self.table_values),
                (6, results),
            ],
            self.capacity,
            len(keys),
            capacity_index=4,
            count_index=5,
        )
        return self._read(results, len(keys))

    def lookup(self, keys):
        keys = np.asarray(keys, dtype=np.uint32).reshape(-1)

        if np.any(keys >= LOCKED_KEY):
            raise ValueError("Reserved keys cannot be queried")
        if len(keys) == 0:
            return (
                np.empty(0, dtype=np.uint32),
                np.empty(0, dtype=np.uint32),
            )

        query_buffer = self._upload(keys)
        output_values = self._buffer(len(keys) * 4)
        found = self._buffer(len(keys) * 4)

        self._dispatch(
            self.lookup_pipeline,
            [
                (0, query_buffer),
                (1, self.table_keys),
                (2, self.table_values),
                (5, output_values),
                (6, found),
            ],
            self.capacity,
            len(keys),
            capacity_index=3,
            count_index=4,
        )
        return (
            self._read(output_values, len(keys)),
            self._read(found, len(keys)),
        )
