param(
    [Parameter(Mandatory = $true)]
    [string]$MsiPath
)

$ErrorActionPreference = "Stop"

function Invoke-MsiScalarQuery {
    param(
        $Database,
        [string]$Query
    )

    $view = $Database.GetType().InvokeMember("OpenView", "InvokeMethod", $null, $Database, @($Query))
    $view.GetType().InvokeMember("Execute", "InvokeMethod", $null, $view, $null) | Out-Null
    $record = $view.GetType().InvokeMember("Fetch", "InvokeMethod", $null, $view, $null)
    if (-not $record) {
        return $null
    }
    return $record.StringData(1)
}

$ResolvedMsiPath = (Resolve-Path $MsiPath).Path
$installer = New-Object -ComObject WindowsInstaller.Installer
$database = $installer.GetType().InvokeMember("OpenDatabase", "InvokeMethod", $null, $installer, @($ResolvedMsiPath, 0))

$fileName = Invoke-MsiScalarQuery -Database $database -Query "SELECT `FileName` FROM `File` WHERE `File`='ClawExe'"
if ($fileName -notlike "claw.exe*") {
    throw "Missing claw.exe file entry in MSI"
}

$installRootName = Invoke-MsiScalarQuery -Database $database -Query "SELECT `DefaultDir` FROM `Directory` WHERE `Directory`='INSTALLROOT'"
if (-not $installRootName -or $installRootName -notmatch "Claw") {
    throw "Missing Claw install root directory in MSI"
}

$binDirName = Invoke-MsiScalarQuery -Database $database -Query "SELECT `DefaultDir` FROM `Directory` WHERE `Directory`='BIN_DIR'"
if ($binDirName -ne "bin") {
    throw "Missing bin directory in MSI"
}

$environmentValue = Invoke-MsiScalarQuery -Database $database -Query "SELECT `Value` FROM `Environment` WHERE `Environment`='ClawBinOnPath'"
if ($environmentValue -notmatch "\[BIN_DIR\]") {
    throw "Missing machine PATH mutation for the Claw bin directory"
}

Write-Output "MSI inspection passed"
