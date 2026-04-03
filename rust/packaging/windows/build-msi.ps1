param(
    [Parameter(Mandatory = $true)]
    [string]$PayloadDir,
    [Parameter(Mandatory = $true)]
    [string]$Version,
    [Parameter(Mandatory = $true)]
    [string]$OutFile
)

$ErrorActionPreference = "Stop"

function Convert-ToMsiVersion {
    param([string]$InputVersion)

    $parts = $InputVersion.Split(".")
    if ($parts.Count -eq 3) {
        return "$InputVersion.0"
    }
    return $InputVersion
}

$WixSource = Join-Path $PSScriptRoot "claw.wxs"
$UpgradeCode = "5B789F3F-6E13-4D77-98D5-1B8D6DEB5F51"
$ProductVersion = Convert-ToMsiVersion -InputVersion $Version
$ResolvedPayloadDir = (Resolve-Path $PayloadDir).Path
$OutDir = Split-Path -Parent $OutFile
if ([string]::IsNullOrWhiteSpace($OutDir)) {
    $OutDir = (Get-Location).Path
}
New-Item -ItemType Directory -Force -Path $OutDir | Out-Null
$ResolvedOutDir = (Resolve-Path $OutDir).Path
$ResolvedOutFile = Join-Path $ResolvedOutDir (Split-Path -Leaf $OutFile)

& wix build `
    $WixSource `
    -arch x64 `
    -d PayloadDir="$ResolvedPayloadDir" `
    -d ProductVersion="$ProductVersion" `
    -d UpgradeCode="$UpgradeCode" `
    -o $ResolvedOutFile 2>&1 | ForEach-Object {
        [Console]::Error.WriteLine($_)
    }

if ($LASTEXITCODE -ne 0) {
    throw "wix build failed with exit code $LASTEXITCODE"
}

Write-Output $ResolvedOutFile
