$ErrorActionPreference = 'Stop'

$Root = Split-Path -Parent $MyInvocation.MyCommand.Path
$MetadataPath = Join-Path $Root 'cargo-metadata-1.6.8.json'
$OutDir = Join-Path $Root 'THIRD_PARTY_LICENSES_1.6.8'
$ReportPath = Join-Path $Root 'THIRD_PARTY_LICENSES_REPORT_1.6.8.txt'
$ZipPath = Join-Path $Root 'THIRD_PARTY_LICENSES_1.6.8.zip'

if (Test-Path $OutDir) { Remove-Item $OutDir -Recurse -Force }
New-Item -ItemType Directory -Path $OutDir | Out-Null

if (Test-Path $MetadataPath) {
    $meta = Get-Content $MetadataPath -Raw -Encoding UTF8 | ConvertFrom-Json
} else {
    Write-Host 'cargo-metadata-1.6.8.json not found; generating metadata with cargo metadata --locked...' -ForegroundColor Cyan
    $metadataJson = & cargo metadata --locked --format-version 1
    if ($LASTEXITCODE -ne 0) { throw 'cargo metadata --locked failed.' }
    $meta = $metadataJson | ConvertFrom-Json
}
$report = New-Object System.Collections.Generic.List[string]
$report.Add('Media Backup Manager 1.6.8 - third-party upstream license-file collection')
$report.Add("Generated: $(Get-Date -Format 'yyyy-MM-dd HH:mm:ss K')")
$report.Add('')

$packagesChecked = 0
$packagesWithFiles = 0
$packagesMissing = 0
$filesCopied = 0

$patterns = @(
    'LICENSE*', 'LICENCE*', 'COPYING*', 'NOTICE*', 'COPYRIGHT*',
    'OFL*', 'UFL*', '*FONT*LICENSE*', '*FONT*LICENCE*'
)

foreach ($pkg in ($meta.packages | Sort-Object name, version)) {
    if ($pkg.name -eq 'media_backup_manager') { continue }
    $packagesChecked++

    $manifest = [string]$pkg.manifest_path
    $pkgDir = Split-Path -Parent $manifest
    $safeName = (([string]$pkg.name) -replace '[^A-Za-z0-9._-]', '_')
    $safeVersion = (([string]$pkg.version) -replace '[^A-Za-z0-9._+-]', '_')
    $dest = Join-Path $OutDir "$safeName-$safeVersion"

    if (-not (Test-Path $pkgDir)) {
        $packagesMissing++
        $report.Add("MISSING PACKAGE DIRECTORY`t$($pkg.name)`t$($pkg.version)`t$pkgDir")
        continue
    }

    $found = @()
    foreach ($pattern in $patterns) {
        $found += Get-ChildItem -Path $pkgDir -File -Recurse -Filter $pattern -ErrorAction SilentlyContinue |
            Where-Object { $_.FullName -notmatch '[\\/]target[\\/]' }
    }
    $found = $found | Sort-Object FullName -Unique

    if (-not $found -or $found.Count -eq 0) {
        $packagesMissing++
        $report.Add("NO LICENSE-LIKE FILE FOUND`t$($pkg.name)`t$($pkg.version)`t$($pkg.license)")
        continue
    }

    New-Item -ItemType Directory -Path $dest | Out-Null
    $packagesWithFiles++
    $i = 0
    foreach ($file in $found) {
        $i++
        $relative = $file.FullName.Substring($pkgDir.Length).TrimStart([char]'\',[char]'/')
        $flat = ($relative -replace '[\\/:*?"<>|]', '__')
        if ([string]::IsNullOrWhiteSpace($flat)) { $flat = "license_$i.txt" }
        $target = Join-Path $dest $flat
        Copy-Item $file.FullName $target -Force
        $filesCopied++
        $report.Add("COPIED`t$($pkg.name)`t$($pkg.version)`t$($pkg.license)`t$relative")
    }
}

$report.Insert(3, "Packages checked: $packagesChecked")
$report.Insert(4, "Packages with license-like files: $packagesWithFiles")
$report.Insert(5, "Packages without collected files or missing source dir: $packagesMissing")
$report.Insert(6, "Files copied: $filesCopied")
$report.Insert(7, '')
$report | Set-Content $ReportPath -Encoding UTF8

if (Test-Path $ZipPath) { Remove-Item $ZipPath -Force }

# ZIP timestamps must be 1980 or newer. Some upstream package files carry
# timestamps that PowerShell's Compress-Archive cannot represent.
$safeTimestamp = Get-Date '2000-01-01T00:00:00'
Get-ChildItem -Path $OutDir -Recurse -Force | ForEach-Object {
    try {
        $_.LastWriteTime = $safeTimestamp
        $_.LastWriteTimeUtc = $safeTimestamp.ToUniversalTime()
    } catch {
        # Timestamp normalization is only needed for ZIP compatibility.
    }
}

Compress-Archive -Path (Join-Path $OutDir '*') -DestinationPath $ZipPath -CompressionLevel Optimal

Write-Host ''
Write-Host 'Third-party license collection finished.' -ForegroundColor Green
Write-Host "Folder : $OutDir"
Write-Host "Report : $ReportPath"
Write-Host "ZIP    : $ZipPath"
Write-Host "Checked: $packagesChecked packages"
Write-Host "Found  : $packagesWithFiles packages with license-like files"
Write-Host "Missing: $packagesMissing packages without collected files / missing source directory"
Write-Host "Copied : $filesCopied files"
Write-Host ''
Write-Host 'Please upload the ZIP and report for final review.' -ForegroundColor Yellow
