pub fn sample()->(Option<f64>,Option<String>){
    let mut best:Option<(u8,f64,String)>=None;
    if let Ok(entries)=std::fs::read_dir("/sys/class/hwmon") {
        for e in entries.flatten(){
            let p=e.path();let name=std::fs::read_to_string(p.join("name")).unwrap_or_default();
            let name=name.trim(); if !["coretemp","k10temp","zenpower"].contains(&name){continue;}
            if let Ok(children)=std::fs::read_dir(&p){
                for f in children.flatten(){
                    let file=f.file_name();let file=file.to_string_lossy();
                    if !file.starts_with("temp")||!file.ends_with("_input"){continue;}
                    let stem=file.trim_end_matches("_input");
                    let label=std::fs::read_to_string(p.join(format!("{stem}_label"))).unwrap_or_default();
                    let label=label.trim();
                    let rank=if label=="Tdie"||label.starts_with("Package id"){3}else if label=="Tctl"{2}else{1};
                    if let Ok(v)=std::fs::read_to_string(f.path()).unwrap_or_default().trim().parse::<f64>(){
                        let c=v/1000.0;if !(-40.0..=150.0).contains(&c){continue;}
                        if best.as_ref().map_or(true,|(r,t,_)|rank>*r||(rank==*r&&c>*t)){
                            best=Some((rank,c,format!("{name}:{}",if label.is_empty(){stem}else{label})));
                        }
                    }
                }
            }
        }
    }
    match best{Some((_,v,l))=>(Some(v),Some(l)),None=>(None,None)}
}
