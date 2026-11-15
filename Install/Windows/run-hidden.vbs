Option Explicit
Dim shell, args, cmd
Set shell=CreateObject("WScript.Shell")
Set args=WScript.Arguments
If args.Count <> 2 Then WScript.Quit 2
cmd=Chr(34) & args(0) & Chr(34) & " " & args(1)
WScript.Quit shell.Run(cmd,0,True)
