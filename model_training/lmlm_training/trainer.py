"""Generic native PyTorch train/evaluate loops; LMLM supplies the actual model."""
from __future__ import annotations
from dataclasses import dataclass
from typing import Callable, Iterable
import math
import torch
from torch import nn
from .device import DevicePlan, resolve_device_plan

@dataclass(frozen=True)
class TrainConfig:
    epochs: int = 1
    accumulation_steps: int = 1
    max_grad_norm: float | None = None
    device: str = "auto"
    precision: str = "auto"
    def __post_init__(self):
        if self.epochs < 1 or self.accumulation_steps < 1:
            raise ValueError("epochs and accumulation_steps must be positive")
        if self.max_grad_norm is not None and self.max_grad_norm <= 0:
            raise ValueError("max_grad_norm must be positive")

@dataclass
class TrainResult:
    history: list[dict[str, float]]
    global_step: int
    device_plan: DevicePlan

def _move(value, device):
    if isinstance(value, torch.Tensor):
        return value.to(device, non_blocking=device.type == "cuda")
    if isinstance(value, tuple):
        return tuple(_move(v, device) for v in value)
    if isinstance(value, list):
        return [_move(v, device) for v in value]
    if isinstance(value, dict):
        return {k: _move(v, device) for k, v in value.items()}
    return value

def _batch(batch, device):
    if isinstance(batch, dict) and "inputs" in batch and "targets" in batch:
        x, y = batch["inputs"], batch["targets"]
    elif isinstance(batch, (tuple, list)) and len(batch) >= 2:
        x, y = batch[0], batch[1]
    else:
        raise ValueError("batch must be (inputs, targets) or contain inputs/targets keys")
    x, y = _move(x, device), _move(y, device)
    tensor = x if isinstance(x, torch.Tensor) else (x[0] if isinstance(x, (tuple, list)) else None)
    if not isinstance(tensor, torch.Tensor) or tensor.ndim == 0:
        raise ValueError("cannot infer batch size from inputs")
    return x, y, int(tensor.shape[0])

def evaluate(model: nn.Module, loader: Iterable, loss_fn: Callable, *,
             device="cpu", precision="fp32"):
    plan = resolve_device_plan(str(device), precision)
    model.eval()
    total, count, batches = 0.0, 0, 0
    with torch.inference_mode():
        for batch in loader:
            x, y, size = _batch(batch, plan.device)
            with plan.autocast():
                loss = loss_fn(model(x), y)
            if loss.ndim != 0 or not bool(torch.isfinite(loss)):
                raise ValueError("loss_fn must return a finite scalar mean loss")
            total += float(loss.detach()) * size
            count += size
            batches += 1
    return {"loss": total / count if count else math.nan,
            "samples": float(count), "batches": float(batches)}

def train(model: nn.Module, train_loader: Iterable, optimizer, loss_fn: Callable, *,
          config=TrainConfig(), validation_loader=None, scheduler=None,
          checkpoint_manager=None, checkpoint_name="last.pt", start_epoch=0,
          global_step=0, on_step=None):
    plan = resolve_device_plan(config.device, config.precision)
    model.to(plan.device)
    scaler = plan.make_grad_scaler()
    history = []
    for epoch in range(start_epoch, start_epoch + config.epochs):
        model.train()
        optimizer.zero_grad(set_to_none=True)
        loss_sum = 0.0
        samples = pending_samples = pending_batches = steps = 0
        for batch in train_loader:
            x, y, size = _batch(batch, plan.device)
            with plan.autocast():
                loss = loss_fn(model(x), y)
            if loss.ndim != 0 or not bool(torch.isfinite(loss)):
                raise ValueError("loss_fn must return a finite scalar mean loss")
            (scaler.scale(loss * size) if scaler else loss * size).backward()
            loss_sum += float(loss.detach()) * size
            samples += size
            pending_samples += size
            pending_batches += 1
            if pending_batches == config.accumulation_steps:
                if scaler:
                    scaler.unscale_(optimizer)
                for p in model.parameters():
                    if p.grad is not None:
                        p.grad.div_(pending_samples)
                if config.max_grad_norm is not None:
                    torch.nn.utils.clip_grad_norm_(model.parameters(), config.max_grad_norm,
                                                   error_if_nonfinite=True)
                if scaler:
                    old_scale = scaler.get_scale()
                    scaler.step(optimizer); scaler.update()
                    updated = scaler.get_scale() >= old_scale
                else:
                    optimizer.step(); updated = True
                optimizer.zero_grad(set_to_none=True)
                pending_samples = pending_batches = 0
                if updated:
                    global_step += 1; steps += 1
                    if scheduler is not None: scheduler.step()
                    if on_step is not None: on_step(global_step, float(loss.detach()))
        if pending_batches:
            if scaler: scaler.unscale_(optimizer)
            for p in model.parameters():
                if p.grad is not None: p.grad.div_(pending_samples)
            if config.max_grad_norm is not None:
                torch.nn.utils.clip_grad_norm_(model.parameters(), config.max_grad_norm,
                                               error_if_nonfinite=True)
            if scaler:
                old_scale = scaler.get_scale()
                scaler.step(optimizer); scaler.update()
                updated = scaler.get_scale() >= old_scale
            else:
                optimizer.step(); updated = True
            optimizer.zero_grad(set_to_none=True)
            if updated:
                global_step += 1; steps += 1
                if scheduler is not None: scheduler.step()
        row = {"epoch": float(epoch + 1),
               "train_loss": loss_sum / samples if samples else math.nan,
               "optimizer_steps": float(steps)}
        if validation_loader is not None:
            row["validation_loss"] = evaluate(
                model, validation_loader, loss_fn, device=str(plan.device),
                precision=plan.precision)["loss"]
        history.append(row)
        if checkpoint_manager is not None:
            checkpoint_manager.save(checkpoint_name, model=model, optimizer=optimizer,
                                    scheduler=scheduler, scaler=scaler, epoch=epoch + 1,
                                    global_step=global_step,
                                    metadata={"device": str(plan.device), "precision": plan.precision})
    return TrainResult(history, global_step, plan)
