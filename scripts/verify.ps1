$ErrorActionPreference = "Stop"
$Root = (Resolve-Path (Join-Path $PSScriptRoot "..")).Path
Push-Location $Root
try {
    Write-Host "==> Rust formatting"
    cargo fmt --all -- --check
    if ($LASTEXITCODE -ne 0) { throw "cargo fmt failed" }

    Write-Host "==> Clippy (all workspace targets)"
    cargo clippy --workspace --all-targets -- -D warnings
    if ($LASTEXITCODE -ne 0) { throw "cargo clippy failed" }

    Write-Host "==> Workspace tests"
    cargo test --workspace
    if ($LASTEXITCODE -ne 0) { throw "cargo test failed" }

    Write-Host "LMLMxtend verification completed successfully."
}
finally {
    Pop-Location
}
