import nki
import nki.isa as nisa
import nki.language as nl

@nki.jit
def indirect_gather_kernel(data, indices):
    P, F = indices.shape[0], data.shape[1]
    output = nl.ndarray((P, F), dtype=data.dtype, buffer=nl.shared_hbm)

    idx = nl.ndarray((P, 1), dtype=nl.uint32, buffer=nl.sbuf)
    nisa.dma_copy(dst=idx, src=indices)

    dst = nl.ndarray((P, F), dtype=data.dtype, buffer=nl.sbuf)
    nisa.dma_copy(
        dst=dst,
        src=data.ap(
            pattern=[[F, P], [1, F]],
            vector_offset=idx,
            indirect_dim=0,
        ),
    )

    nisa.dma_copy(dst=output, src=dst)
    return output
