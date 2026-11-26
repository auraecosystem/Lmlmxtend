# Native LMLM training

This package adds reusable PyTorch training services under
`model_training/lmlm_training/`. It is deliberately model-agnostic: the LMLM
model implementation owns architecture, tokenizer, data pipeline and model
artifacts. The package accepts an existing `torch.nn.Module`, optimizer,
data loader and loss function.

## Included

- Automatic CUDA → Apple MPS → CPU device selection.
- Automatic BF16/FP16 selection on CUDA when supported; FP32 on MPS/CPU by default.
- Sample-weighted gradient accumulation, gradient clipping and evaluation.
- Atomic resumable checkpoints for model/optimizer/scheduler/scaler and RNG state.
- Inference microbenchmarks with warmup, latency percentiles, throughput and CUDA peak memory.

## Example

```python
from model_training.lmlm_training import TrainConfig, CheckpointManager, train

result = train(
    lmlm_model, train_loader, optimizer, loss_fn,
    config=TrainConfig(epochs=3, accumulation_steps=4,
                       device="auto", precision="auto", max_grad_norm=1.0),
    validation_loader=validation_loader,
    checkpoint_manager=CheckpointManager("artifacts/checkpoints"),
)
```

The identifiers above are supplied by the LMLM model and dataset implementation;
this package does not invent a model constructor or tokenizer contract.

## Boundary with LMLMxtend

The Rust processor bridge is an inference boundary, not a training control
protocol. Training runs in the LMLM-owned Python/PyTorch environment. Export
approved model artifacts through the LMLM model-manifest pipeline; do not send
optimizer state or training commands over `lmlm.processor.v1`.

## Security and verification

PyTorch checkpoints use pickle serialization. Load only trusted local checkpoint
files. Run `python -m pytest -q tests/test_lmlm_training.py`. GPU precision and
performance require validation on the actual target device and model.
