$ErrorActionPreference = "Stop"
$Root = (Resolve-Path (Join-Path $PSScriptRoot "..")).Path
Push-Location $Root
$TempRoot = Join-Path "extensions" ".lmlmxtend-smoke-$PID"
try {
    Write-Host "==> Extension runtime example"
    cargo run --example basic_extension -p lmlmxtend-runtime
    if ($LASTEXITCODE -ne 0) { throw "Extension example failed" }

    Write-Host "==> Agent runtime example"
    cargo run --example basic_agent -p lmlmxtend-agent-runtime
    if ($LASTEXITCODE -ne 0) { throw "Agent example failed" }

    Write-Host "==> Extension Forge help"
    cargo run -p lmlmxtend-extension-forge -- help
    if ($LASTEXITCODE -ne 0) { throw "Forge help failed" }

    if (Test-Path $TempRoot) { throw "Temporary smoke directory already exists: $TempRoot" }
    New-Item -ItemType Directory -Path "extensions" -Force | Out-Null

    Write-Host "==> Generate and compile a temporary extension"
    cargo run -p lmlmxtend-extension-forge -- new --name smoke-test --category creative --capability creative.compose --root $TempRoot
    if ($LASTEXITCODE -ne 0) { throw "Extension generation failed" }

    cargo test --manifest-path (Join-Path $TempRoot "creative/smoke-test/Cargo.toml")
    if ($LASTEXITCODE -ne 0) { throw "Generated extension tests failed" }

    Write-Host "LMLMxtend smoke checks completed successfully."
}
finally {
    if (Test-Path $TempRoot) { Remove-Item -Recurse -Force $TempRoot }
    Pop-Location
}
