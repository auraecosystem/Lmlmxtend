# LMLMxtend developer scripts

Run these commands from the repository root. A stable Rust toolchain is required.

## Verify source and tests

- Linux/macOS: `bash scripts/verify.sh`
- Windows PowerShell: `pwsh -File scripts/verify.ps1`
- Make: `make verify`

This runs rustfmt in check mode, Clippy across all workspace targets with warnings denied, and the complete workspace test suite.

## Smoke-test executable examples and Extension Forge

- Linux/macOS: `bash scripts/smoke.sh`
- Windows PowerShell: `pwsh -File scripts/smoke.ps1`
- Make: `make smoke`

This runs the extension and agent examples, checks the Forge help command, creates a temporary extension, compiles and tests it, then removes the temporary files.

## Scaffold an extension

Linux/macOS:

```bash
bash scripts/new-extension.sh --name scene-composer --category creative --capability creative.compose
```

Windows PowerShell:

```powershell
pwsh -File scripts/new-extension.ps1 -Name scene-composer -Category creative -Capability creative.compose
```

The generated extension is placed under `extensions/<category>/<name>`. The Forge refuses to overwrite an existing destination. Generated extensions are standalone Cargo workspaces and are not automatically added to the root workspace.

## CI

GitHub Actions invokes the same verification and smoke scripts, so local and hosted checks use the same commands.
