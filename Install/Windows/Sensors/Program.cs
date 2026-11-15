using LibreHardwareMonitor.Hardware;
using System.Text.Json;
using System.Threading;
var dir=Path.Combine(Environment.GetFolderPath(Environment.SpecialFolder.LocalApplicationData),"NEXUS-Hardware-Control","state");
Directory.CreateDirectory(dir);
using var mutex=new Mutex(true,@"Local\NHC-Sensors",out bool own);
if(!own)return;
var pc=new Computer{IsCpuEnabled=true,IsGpuEnabled=true,IsMotherboardEnabled=true,IsControllerEnabled=true,IsStorageEnabled=true};
try{
 pc.Open();
 while(true){
  try{
   var all=new List<(IHardware h,ISensor s)>();
   void Visit(IHardware h){try{h.Update();foreach(var s in h.Sensors)if(s.Value.HasValue&&float.IsFinite(s.Value.Value))all.Add((h,s));foreach(var sub in h.SubHardware)Visit(sub);}catch{}}
   foreach(var h in pc.Hardware)Visit(h);
   float? Find(IEnumerable<(IHardware h,ISensor s)> a,SensorType t,params string[] names){
    foreach(var n in names){var v=a.FirstOrDefault(x=>x.s.SensorType==t&&x.s.Name.Equals(n,StringComparison.OrdinalIgnoreCase));if(v.s!=null)return v.s.Value;}return null;}
   var cpu=all.Where(x=>x.h.HardwareType==HardwareType.Cpu).ToList();
   var gpu=all.Where(x=>x.h.HardwareType==HardwareType.GpuNvidia||x.h.HardwareType==HardwareType.GpuAmd||x.h.HardwareType==HardwareType.GpuIntel).GroupBy(x=>x.h.Identifier.ToString()).FirstOrDefault();
   var g=gpu?.ToList()??new List<(IHardware h,ISensor s)>();
   object Rows(SensorType type)=>all.Where(x=>x.s.SensorType==type).Select(x=>new{label=x.h.Name+" / "+x.s.Name,value=x.s.Value}).ToArray();
   var cpuTemp=Find(cpu,SensorType.Temperature,"CPU Package","Core (Tctl/Tdie)","Core (Tdie)","CPU Core");
   var cpuPower=Find(cpu,SensorType.Power,"CPU Package","Package");
   var data=new{cpu_temp=cpuTemp,cpu_power=cpuPower,fan_rpm=all.FirstOrDefault(x=>x.s.SensorType==SensorType.Fan&&x.s.Value>0).s?.Value,
    fans=Rows(SensorType.Fan),temperatures=Rows(SensorType.Temperature),voltages=Rows(SensorType.Voltage),
    gpu=new{usage_percent=Find(g,SensorType.Load,"GPU Core","GPU D3D 3D"),temperature_c=Find(g,SensorType.Temperature,"GPU Core"),frequency_mhz=Find(g,SensorType.Clock,"GPU Core"),power_w=Find(g,SensorType.Power,"GPU Package","GPU Power","GPU Total Board Power"),vram_used_mib=Find(g,SensorType.SmallData,"GPU Memory Used"),vram_total_mib=Find(g,SensorType.SmallData,"GPU Memory Total")}};
   var path=Path.Combine(dir,"sensors.json");var tmp=path+"."+Environment.ProcessId+".tmp";
   File.WriteAllText(tmp,JsonSerializer.Serialize(new{version=1,timestamp=DateTimeOffset.UtcNow.ToUnixTimeMilliseconds()/1000.0,data}));
   File.Move(tmp,path,true);
  }catch(Exception e){try{File.WriteAllText(Path.Combine(dir,"sensor-error.txt"),e.Message);}catch{}}
  Thread.Sleep(1500);
 }
}finally{pc.Close();}
