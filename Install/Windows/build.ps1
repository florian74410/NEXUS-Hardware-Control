# Developer/CI only: Rust MSVC toolchain and .NET 8 SDK required.
$ErrorActionPreference='Stop'
$repo=(Resolve-Path (Join-Path $PSScriptRoot '../..')).Path
$out=Join-Path $repo 'dist\NHC-Windows-x64'
if(Test-Path $out){Remove-Item $out -Recurse -Force}
New-Item -ItemType Directory -Force -Path (Join-Path $out 'payload\Sensors') | Out-Null
Push-Location (Join-Path $repo 'Core')
try {
 cargo test --target x86_64-pc-windows-msvc
 if($LASTEXITCODE -ne 0){throw 'cargo test failed'}
 cargo build --release --target x86_64-pc-windows-msvc
 if($LASTEXITCODE -ne 0){throw 'cargo build failed'}
 Copy-Item 'target\x86_64-pc-windows-msvc\release\nhc.exe' (Join-Path $out 'payload')
}finally{Pop-Location}
dotnet publish (Join-Path $PSScriptRoot 'Sensors\NhcSensors.csproj') -c Release -r win-x64 --self-contained true -o (Join-Path $out 'payload\Sensors')
if($LASTEXITCODE -ne 0){throw 'sensor build failed'}
foreach($name in @('Install-NHC.cmd','install.ps1','uninstall.ps1','run-hidden.vbs','README.md')){Copy-Item (Join-Path $PSScriptRoot $name) $out}
# Include upstream licensing notice; dependency license inventory must be reviewed before stable release.
Copy-Item (Join-Path $PSScriptRoot 'THIRD-PARTY.txt') $out
$zip=Join-Path $repo 'dist\NHC-Windows-x64.zip'
Compress-Archive -Path (Join-Path $out '*') -DestinationPath $zip -Force
Get-FileHash $zip -Algorithm SHA256
