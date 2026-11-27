import torch

from model_training.lmlm_training.amp import (
    autocast,
    custom_fwd,
    custom_bwd,
)

class MyOperation(torch.autograd.Function):

    @staticmethod
    @custom_fwd(cast_inputs=torch.float16)
    def forward(ctx, x):
        return x * 2

    @staticmethod
    @custom_bwd
    def backward(ctx, grad):
        return grad * 2

with autocast(dtype=torch.float16):
    output = MyOperation.apply(
        torch.randn(32, 32, device="cuda")
    )
