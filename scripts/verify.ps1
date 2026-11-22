$ErrorActionPreference = "Stop"
$Root = (Resolve-Path (Join-Path $PSScriptRoot "..")).Path
$FileIntelligence = "extensions/productivity/file-intelligence/Cargo.toml"
Push-Location $Root
try {
    Write-Host "==> Rust workspace formatting"
    cargo fmt --all -- --check
    if ($LASTEXITCODE -ne 0) { throw "Workspace cargo fmt failed" }

    Write-Host "==> File Intelligence formatting"
    cargo fmt --manifest-path $FileIntelligence -- --check
    if ($LASTEXITCODE -ne 0) { throw "File Intelligence cargo fmt failed" }

    Write-Host "==> Clippy (workspace targets)"
    cargo clippy --workspace --all-targets -- -D warnings
    if ($LASTEXITCODE -ne 0) { throw "Workspace cargo clippy failed" }

    Write-Host "==> Clippy (File Intelligence)"
    cargo clippy --manifest-path $FileIntelligence --all-targets -- -D warnings
    if ($LASTEXITCODE -ne 0) { throw "File Intelligence cargo clippy failed" }

    Write-Host "==> File Intelligence tests"
    cargo test --manifest-path $FileIntelligence
    if ($LASTEXITCODE -ne 0) { throw "File Intelligence cargo test failed" }

    Write-Host "==> Workspace tests"
    cargo test --workspace
    if ($LASTEXITCODE -ne 0) { throw "Workspace cargo test failed" }

    Write-Host "LMLMxtend verification completed successfully."
}
finally {
    Pop-Location
}
