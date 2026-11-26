import torch
from torch import nn
from model_training.lmlm_training import (
    TrainConfig, CheckpointManager, load_checkpoint, train, evaluate,
    resolve_device_plan, benchmark,
)

def loader():
    x = torch.randn(24, 4)
    y = x.sum(1, keepdim=True)
    return torch.utils.data.DataLoader(torch.utils.data.TensorDataset(x, y), batch_size=5)

def test_device_auto_cpu():
    plan = resolve_device_plan("cpu", "auto")
    assert plan.device.type == "cpu" and plan.precision == "fp32"

def test_training_checkpoint_and_validation(tmp_path):
    model = nn.Linear(4, 1)
    opt = torch.optim.SGD(model.parameters(), lr=.05)
    manager = CheckpointManager(tmp_path)
    result = train(model, loader(), opt, nn.MSELoss(),
        config=TrainConfig(epochs=2, accumulation_steps=2, device="cpu"),
        validation_loader=loader(), checkpoint_manager=manager)
    assert len(result.history) == 2 and result.global_step > 0
    assert (tmp_path / "last.pt").exists()

def test_checkpoint_roundtrip(tmp_path):
    model = nn.Linear(4, 1)
    opt = torch.optim.Adam(model.parameters())
    path = CheckpointManager(tmp_path).save("state.pt", model=model, optimizer=opt,
                                            epoch=2, global_step=7)
    restored = nn.Linear(4, 1)
    state = load_checkpoint(path, model=restored)
    assert state["epoch"] == 2 and state["global_step"] == 7
    for a, b in zip(model.parameters(), restored.parameters()):
        assert torch.equal(a, b)

def test_evaluate_counts_samples():
    result = evaluate(nn.Linear(4, 1), loader(), nn.MSELoss(), device="cpu")
    assert result["samples"] == 24 and result["batches"] == 5

def test_benchmark_metrics():
    result = benchmark(nn.Linear(4, 2), torch.randn(3, 4),
                       device="cpu", warmup=1, iterations=3)
    assert result.mean_latency_ms > 0 and result.samples_per_second > 0

def test_invalid_accumulation():
    try: TrainConfig(accumulation_steps=0)
    except ValueError: return
    assert False

def test_nan_loss_rejected():
    model = nn.Linear(4, 1)
    opt = torch.optim.SGD(model.parameters(), lr=.01)
    def loss_fn(a, b): return a.sum() * torch.tensor(float("nan"))
    try:
        train(model, loader(), opt, loss_fn, config=TrainConfig(device="cpu"))
    except ValueError: return
    assert False
