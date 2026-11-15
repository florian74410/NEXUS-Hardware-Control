#Requires -Version 5.1
$ErrorActionPreference='Stop'
if (-not [Environment]::Is64BitOperatingSystem) { throw 'Windows x64 required' }
$root=Join-Path $env:LOCALAPPDATA 'NEXUS-Hardware-Control'
$bin=Join-Path $root 'bin'
$payload=Join-Path $PSScriptRoot 'payload'
if (!(Test-Path (Join-Path $payload 'nhc.exe')) -or !(Test-Path (Join-Path $payload 'Sensors\nhc-sensors.exe'))) { throw 'Use the compiled Windows package, not the source ZIP.' }
$sid=[System.Security.Principal.WindowsIdentity]::GetCurrent().User.Value
$user=[System.Security.Principal.WindowsIdentity]::GetCurrent().Name
$names=@("NHC-Core-$sid","NHC-Touch-$sid","NHC-Sensors-$sid")
foreach($name in $names){if(Get-ScheduledTask -TaskName $name -ErrorAction SilentlyContinue){Stop-ScheduledTask -TaskName $name -ErrorAction SilentlyContinue;Unregister-ScheduledTask -TaskName $name -Confirm:$false}}
New-Item -ItemType Directory -Force -Path $bin,(Join-Path $root 'state') | Out-Null
Copy-Item (Join-Path $payload '*') $bin -Recurse -Force
Copy-Item (Join-Path $PSScriptRoot 'uninstall.ps1') $root -Force
Copy-Item (Join-Path $PSScriptRoot 'run-hidden.vbs') $root -Force
& (Join-Path $bin 'nhc.exe') --version
if($LASTEXITCODE -ne 0){throw 'Core executable failed'}
$principal=New-ScheduledTaskPrincipal -UserId $user -LogonType Interactive -RunLevel Limited
$trigger=New-ScheduledTaskTrigger -AtLogOn -User $user
$settings=New-ScheduledTaskSettingsSet -MultipleInstances IgnoreNew -ExecutionTimeLimit ([TimeSpan]::Zero) -AllowStartIfOnBatteries -DontStopIfGoingOnBatteries -StartWhenAvailable -RestartCount 3 -RestartInterval (New-TimeSpan -Minutes 1)
$wscript=Join-Path $env:SystemRoot 'System32\wscript.exe'
foreach($job in @(@{Name=$names[0];Exe='nhc.exe';Args='service run'},@{Name=$names[1];Exe='nhc.exe';Args='usb auto'},@{Name=$names[2];Exe='Sensors\nhc-sensors.exe';Args=''})){
 $exe=Join-Path $bin $job.Exe
 $launcher=Join-Path $root 'run-hidden.vbs'
 $action=New-ScheduledTaskAction -Execute $wscript -Argument ('//B //Nologo "{0}" "{1}" "{2}"' -f $launcher,$exe,$job.Args) -WorkingDirectory $bin
 Register-ScheduledTask -TaskName $job.Name -Action $action -Trigger $trigger -Principal $principal -Settings $settings -Force | Out-Null
 Start-ScheduledTask -TaskName $job.Name
}
Write-Host "NHC installed for $user. Plug in NHC Touch. No firmware update required."
Write-Host "Diagnostics: & '$bin\nhc.exe' doctor"
Write-Host "If no COM port appears, install the CH340 vendor driver through Windows Update or WCH."
