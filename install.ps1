$ErrorActionPreference = "Stop"

$Repo = "hyiip/zquery"
$InstallDir = if ($env:INSTALL_DIR) { $env:INSTALL_DIR } else { "$env:USERPROFILE\.local\bin" }
$Artifact = "zquery-windows-x86_64"

# Get latest release tag
$Release = Invoke-RestMethod "https://api.github.com/repos/$Repo/releases/latest"
$Tag = $Release.tag_name
Write-Host "Installing zquery $Tag for Windows x86_64..."

# Download
$TmpDir = New-Item -ItemType Directory -Path (Join-Path $env:TEMP "zquery-install-$(Get-Random)")
try {
    $ZipUrl = "https://github.com/$Repo/releases/download/$Tag/$Artifact.zip"
    $ZipPath = Join-Path $TmpDir "zquery.zip"
    Invoke-WebRequest -Uri $ZipUrl -OutFile $ZipPath
    Expand-Archive -Path $ZipPath -DestinationPath $TmpDir

    # Install binary
    New-Item -ItemType Directory -Force -Path $InstallDir | Out-Null
    Copy-Item (Join-Path $TmpDir "$Artifact.exe") (Join-Path $InstallDir "zquery.exe")
    Write-Host "Installed zquery to $InstallDir\zquery.exe"

    # Install Claude Code skill
    $SkillDir = "$env:USERPROFILE\.claude\skills\zquery"
    New-Item -ItemType Directory -Force -Path $SkillDir | Out-Null
    Copy-Item (Join-Path $TmpDir "skill\SKILL.md") $SkillDir
    Write-Host "Installed Claude Code skill to $SkillDir"

    # Check PATH
    if (-not ($env:PATH -split ";" | Where-Object { $_ -eq $InstallDir })) {
        Write-Host ""
        Write-Host "Note: $InstallDir is not in your PATH. Add it with:"
        Write-Host "  [Environment]::SetEnvironmentVariable('PATH', `"$InstallDir;`" + [Environment]::GetEnvironmentVariable('PATH', 'User'), 'User')"
    }

    Write-Host "Done."
} finally {
    Remove-Item -Recurse -Force $TmpDir
}
