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

$ResolvedPayloadDir = (Resolve-Path $PayloadDir).Path
$ResolvedOutDir = [System.IO.Path]::GetDirectoryName((Resolve-Path -LiteralPath (Split-Path -Parent $OutFile) -ErrorAction SilentlyContinue)?.Path ?? (Split-Path -Parent $OutFile))
if (-not $ResolvedOutDir) {
    $ResolvedOutDir = (Get-Location).Path
}
New-Item -ItemType Directory -Force -Path $ResolvedOutDir | Out-Null
$ResolvedOutFile = Join-Path $ResolvedOutDir (Split-Path -Leaf $OutFile)

$WixSource = Join-Path $PSScriptRoot "claw.wxs"
$UpgradeCode = "5B789F3F-6E13-4D77-98D5-1B8D6DEB5F51"
$ProductVersion = Convert-ToMsiVersion -InputVersion $Version

wix build `
    $WixSource `
    -arch x64 `
    -d PayloadDir="$ResolvedPayloadDir" `
    -d ProductVersion="$ProductVersion" `
    -d UpgradeCode="$UpgradeCode" `
    -o $ResolvedOutFile

Write-Output $ResolvedOutFile
