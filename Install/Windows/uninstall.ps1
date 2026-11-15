$ErrorActionPreference='Stop'
$sid=[System.Security.Principal.WindowsIdentity]::GetCurrent().User.Value
foreach($name in @("NHC-Core-$sid","NHC-Touch-$sid","NHC-Sensors-$sid")){
 if(Get-ScheduledTask -TaskName $name -ErrorAction SilentlyContinue){Stop-ScheduledTask -TaskName $name -ErrorAction SilentlyContinue;Unregister-ScheduledTask -TaskName $name -Confirm:$false}
}
$root=Join-Path $env:LOCALAPPDATA 'NEXUS-Hardware-Control'
Get-Process nhc,nhc-sensors -ErrorAction SilentlyContinue | Where-Object {$_.Path -and $_.Path.StartsWith(($root+'\bin\'),[StringComparison]::OrdinalIgnoreCase)} | Stop-Process -ErrorAction SilentlyContinue
if(Test-Path $root){Remove-Item $root -Recurse -Force}
Write-Host 'NHC removed.'
