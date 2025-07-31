$ErrorActionPreference = "Stop"

# Define output names
$project = "waitonfile"
$releaseDir = "target"

# Build 64-bit
cargo build --release --target x86_64-pc-windows-msvc
Copy-Item "$releaseDir\x86_64-pc-windows-msvc\release\$project.exe" "$project-x64.exe" -Force

# Build 32-bit
cargo build --release --target i686-pc-windows-msvc
Copy-Item "$releaseDir\i686-pc-windows-msvc\release\$project.exe" "$project-x86.exe" -Force

Write-Host "`n✅ Builds complete:"
Write-Host "  • $project-x64.exe"
Write-Host "  • $project-x86.exe"

