#[derive(Default)]
pub struct Cpu { previous: Option<(u64,u64)> }
impl Cpu {
    pub fn sample(&mut self) -> (Option<f64>,Option<f64>) {
        let ticks=std::fs::read_to_string("/proc/stat").ok().and_then(|s| {
            let row=s.lines().next()?;
            let n:Vec<u64>=row.strip_prefix("cpu ")?.split_whitespace().take(8)
                .map(str::parse).collect::<Result<_,_>>().ok()?;
            if n.len()<4 {return None;}
            Some((n.iter().sum::<u64>(),n[3]+n.get(4).copied().unwrap_or(0)))
        });
        let load=match (self.previous,ticks) {
            (Some((t0,i0)),Some((t1,i1))) if t1>t0 && i1>=i0 && i1-i0<=t1-t0 =>
                Some(((t1-t0-(i1-i0)) as f64/(t1-t0) as f64*1000.0).round()/10.0),
            _=>None
        };
        self.previous=ticks;
        let mhz=std::fs::read_to_string("/proc/cpuinfo").ok().and_then(|s|{
            let a:Vec<f64>=s.lines().filter_map(|l|{let (k,v)=l.split_once(':')?;
                if k.trim()=="cpu MHz" {v.trim().parse::<f64>().ok()} else {None}}).filter(|v|v.is_finite()&&*v>0.0).collect();
            if a.is_empty(){None}else{Some((a.iter().sum::<f64>()/a.len() as f64).round())}
        });
        (load,mhz)
    }
}

#[derive(Default)]
pub struct Cores { prev:std::collections::BTreeMap<String,(u64,u64)> }
impl Cores {
 pub fn sample(&mut self)->Vec<serde_json::Value>{
 let clocks:std::collections::BTreeMap<String,f64>=std::fs::read_to_string("/proc/cpuinfo").unwrap_or_default().split("\n\n").filter_map(|block|{
 let mut id=None;let mut mhz=None;
 for l in block.lines(){if let Some((k,v))=l.split_once(':'){match k.trim(){"processor"=>id=Some(v.trim().to_owned()),"cpu MHz"=>mhz=v.trim().parse::<f64>().ok().filter(|x|x.is_finite()),_=>{}}}}
 Some((id?,mhz?))}).collect();
 let mut next=std::collections::BTreeMap::new();let mut out=Vec::new();
 for l in std::fs::read_to_string("/proc/stat").unwrap_or_default().lines(){
 let mut w=l.split_whitespace();let Some(k)=w.next() else{continue};let Some(id)=k.strip_prefix("cpu") else{continue};if id.is_empty()||!id.bytes().all(|b|b.is_ascii_digit()){continue;}
 let n:Vec<u64>=w.take(8).filter_map(|s|s.parse().ok()).collect();if n.len()<4{continue;}
 let t=n.iter().sum::<u64>();let i=n[3]+n.get(4).copied().unwrap_or(0);
 let load=self.prev.get(id).and_then(|&(a,b)|{let dt=t.checked_sub(a)?;let di=i.checked_sub(b)?;if dt==0||di>dt{return None;}Some(100.0*(dt-di) as f64/dt as f64)});
 next.insert(id.to_owned(),(t,i));out.push(serde_json::json!({"logical_cpu":id.parse::<u32>().ok(),"usage_percent":load,"frequency_mhz":clocks.get(id)}));
 }self.prev=next;out
 }
}
