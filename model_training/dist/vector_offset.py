import nki
import nki.isa as nisa
import nki.language as nl

@nki.jit
def indirect_scatter_kernel(src_data, indices, output):
    P, F = src_data.shape

    src = nl.ndarray((P, F), dtype=src_data.dtype, buffer=nl.sbuf)
    nisa.dma_copy(dst=src, src=src_data)

    idx = nl.ndarray((P, 1), dtype=nl.uint32, buffer=nl.sbuf)
    nisa.dma_copy(dst=idx, src=indices)

    nisa.dma_copy(
        dst=output.ap(
            pattern=[[F, P], [1, F]],
            vector_offset=idx,
            indirect_dim=0,
        ),
        src=src,
    )
    return output
