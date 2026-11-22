param(
    [Parameter(Mandatory = $true)][string]$Name,
    [Parameter(Mandatory = $true)][ValidateSet("creative", "language", "vision", "audio", "spatial", "agents")][string]$Category,
    [Parameter(Mandatory = $true)][string]$Capability,
    [string]$Root = "extensions"
)
$ErrorActionPreference = "Stop"
$RepoRoot = (Resolve-Path (Join-Path $PSScriptRoot "..")).Path
Push-Location $RepoRoot
try {
    cargo run -p lmlmxtend-extension-forge -- new --name $Name --category $Category --capability $Capability --root $Root
    if ($LASTEXITCODE -ne 0) { throw "Extension Forge failed" }
}
finally {
    Pop-Location
}
