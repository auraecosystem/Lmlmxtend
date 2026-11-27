from model_training.lmlm_training import custom_fwd, custom_bwd

class MyFunction(torch.autograd.Function):
    @staticmethod
    @custom_fwd(device_type="cuda")
    def forward(ctx, x):
        return x * 2

    @staticmethod
    @custom_bwd
    def backward(ctx, grad):
        return grad * 2
