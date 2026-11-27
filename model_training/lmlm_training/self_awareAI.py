from dataclasses import dataclass, field
from datetime import datetime, timezone
from typing import Any
import json


@dataclass
class SelfAwareAI:
    state: str = "idle"
    memory: list[dict[str, Any]] = field(default_factory=list)
    actions: dict[str, str] = field(default_factory=lambda: {
        "hungry": "eat",
        "tired": "sleep",
        "happy": "do nothing",
    })

    def _remember(self, event: str, value: Any):
        self.memory.append({
            "event": event,
            "value": value,
            "timestamp": datetime.now(
                timezone.utc
            ).isoformat(),
        })

    def perceive(self, observation: str):
        if not isinstance(observation, str):
            raise TypeError("Observation must be a string")

        self.state = observation.strip().lower()
        self._remember("perception", self.state)

    def reflect(self) -> dict:
        return {
            "current_state": self.state,
            "memory_size": len(self.memory),
            "last_event": (
                self.memory[-1] if self.memory else None
            ),
            "available_actions": list(self.actions.values()),
        }

    def act(self) -> str:
        action = self.actions.get(
            self.state,
            "do nothing"
        )

        self._remember("action", action)
        return action

    def introspect(self) -> dict:
        return {
            "identity": self.__class__.__name__,
            "state": self.state,
            "memory_events": len(self.memory),
            "known_states": list(self.actions.keys()),
            "known_actions": list(self.actions.values()),
        }

    def export_memory(self) -> str:
        return json.dumps(
            self.memory,
            indent=2,
            allow_nan=False
        )


if __name__ == "__main__":
    ai = SelfAwareAI()

    for observation in ["hungry", "tired", "happy"]:
        ai.perceive(observation)
        action = ai.act()

        print("Reflection:", ai.reflect())
        print("Action:", action)

    print("Introspection:", ai.introspect())

