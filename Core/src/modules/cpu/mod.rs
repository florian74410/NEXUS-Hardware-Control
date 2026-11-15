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
