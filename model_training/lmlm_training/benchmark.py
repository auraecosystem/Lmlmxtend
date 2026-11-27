"""Repeatable local inference microbenchmark."""
from __future__ import annotations
from dataclasses import dataclass, asdict
import time
import torch
from torch import nn
from .device import resolve_device_plan

@dataclass
class BenchmarkResult:
    device: str
    precision: str
    batch_size: int
    warmup_iterations: int
    measured_iterations: int
    mean_latency_ms: float
    p50_latency_ms: float
    p95_latency_ms: float
    samples_per_second: float
    peak_memory_bytes: int | None
    def to_dict(self):
        return asdict(self)

def benchmark(model: nn.Module, inputs: torch.Tensor, *, device="auto",
              precision="auto", warmup=5, iterations=30):
    if warmup < 0 or iterations < 1:
        raise ValueError("warmup must be >= 0 and iterations >= 1")
    if not isinstance(inputs, torch.Tensor) or inputs.ndim == 0:
        raise ValueError("inputs must be a batched tensor")
    plan = resolve_device_plan(device, precision)
    model.to(plan.device).eval()
    inputs = inputs.to(plan.device)
    if plan.device.type == "cuda":
        torch.cuda.reset_peak_memory_stats(plan.device)
    def run():
        with torch.inference_mode(), plan.autocast():
            model(inputs)
        if plan.device.type == "cuda":
            torch.cuda.synchronize(plan.device)
    for _ in range(warmup): run()
    timings = []
    for _ in range(iterations):
        start = time.perf_counter(); run()
        timings.append((time.perf_counter() - start) * 1000.0)
    values = torch.tensor(timings, dtype=torch.float64)
    mean = float(values.mean())
    peak = torch.cuda.max_memory_allocated(plan.device) if plan.device.type == "cuda" else None
    return BenchmarkResult(str(plan.device), plan.precision, int(inputs.shape[0]), warmup,
        iterations, mean, float(torch.quantile(values, .50)), float(torch.quantile(values, .95)),
        int(inputs.shape[0]) * iterations / (sum(timings) / 1000.0), peak)
