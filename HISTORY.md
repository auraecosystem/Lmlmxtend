# Historical planning notes

This file previously contained early design notes and an initial repository-inspection snapshot. Its proposed directory layout and statements about an empty repository are obsolete and must not be used as the current implementation specification.

For the maintained architecture and implementation status, use:

- [Project overview](docs/README.md)
- [Architecture](docs/ARCHITECTURE.md)
- [Security model](docs/SECURITY.md)
- [Roadmap](docs/ROADMAP.md)

Current responsibility boundary: LMLM is the Large Multimodal Local Model and owns the neural processor; LMLMxtend provides independent extension contracts, orchestration, adapters, and SDK boundaries. Host applications are optional consumers.
