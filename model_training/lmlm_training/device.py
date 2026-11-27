"""Conservative hardware and precision selection."""
from __future__ import annotations
from dataclasses import dataclass
import torch
from contextlib import nullcontext

@dataclass(frozen=True)
class DevicePlan:
    device: torch.device
    precision: str
    amp_enabled: bool
    reason: str

    def autocast(self):
        if not self.amp_enabled:
            return nullcontext()
        dtype = torch.float16 if self.precision == "fp16" else torch.bfloat16
        return torch.autocast(device_type=self.device.type, dtype=dtype)

    def make_grad_scaler(self):
        if self.device.type == "cuda" and self.precision == "fp16" and self.amp_enabled:
            return torch.amp.GradScaler("cuda")
        return None

def resolve_device_plan(device="auto", precision="auto", *, allow_mps=True):
    """Auto: CUDA, then MPS, then CPU. Auto precision: CUDA BF16/FP16, else FP32."""
    if device == "auto":
        if torch.cuda.is_available():
            resolved = torch.device("cuda")
        elif allow_mps and hasattr(torch.backends, "mps") and torch.backends.mps.is_available():
            resolved = torch.device("mps")
        else:
            resolved = torch.device("cpu")
    else:
        resolved = torch.device(device)
    if resolved.type == "cuda" and not torch.cuda.is_available():
        raise RuntimeError("CUDA was requested but is unavailable")
    if resolved.type == "mps" and (not allow_mps or not hasattr(torch.backends, "mps")
                                   or not torch.backends.mps.is_available()):
        raise RuntimeError("MPS was requested but is unavailable")
    if resolved.type not in {"cpu", "cuda", "mps"}:
        raise ValueError(f"unsupported device type: {resolved.type}")
    if precision == "auto":
        if resolved.type == "cuda":
            precision = "bf16" if torch.cuda.is_bf16_supported() else "fp16"
        else:
            precision = "fp32"
    if precision not in {"fp32", "fp16", "bf16"}:
        raise ValueError("precision must be auto, fp32, fp16, or bf16")
    if precision == "fp16" and resolved.type != "cuda":
        raise ValueError("FP16 AMP is enabled only for CUDA")
    if precision == "bf16" and resolved.type == "cuda" and not torch.cuda.is_bf16_supported():
        raise RuntimeError("CUDA device does not support BF16")
    return DevicePlan(resolved, precision, precision != "fp32",
                      f"selected {resolved.type}/{precision}")
