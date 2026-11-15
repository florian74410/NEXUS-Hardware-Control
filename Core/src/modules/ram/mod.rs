pub fn sample()->(Option<u64>,Option<u64>){
    let s=match std::fs::read_to_string("/proc/meminfo"){Ok(s)=>s,Err(_)=>return (None,None)};
    let get=|key:&str|s.lines().find_map(|l|l.strip_prefix(key)?.split_whitespace().next()?.parse::<u64>().ok());
    let total=get("MemTotal:"); let available=get("MemAvailable:");
    (total.zip(available).and_then(|(t,a)|t.checked_sub(a)).map(|v|v/1024),total.map(|v|v/1024))
}
