import torch
from model_training.self_awareness import SelfAwareness

model = torch.nn.Linear(16, 4)

observer = SelfAwareness(model)

observer.start_epoch(1)
observer.record_step(
    torch.tensor(0.42),
    batch_size=32,
)

report = observer.inspect()
print(report)
