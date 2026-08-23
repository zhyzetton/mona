fn main() {
    let conn = rusqlite::Connection::open("library.db").unwrap();
    let mut stmt = conn.prepare("SELECT COUNT(*) FROM media").unwrap();
    let count: i64 = stmt.query_row([], |r| r.get(0)).unwrap();
    println!("media count: {}", count);
    // 测试 find_all 的查询
    let mut stmt = conn.prepare("SELECT id,title,year,overview,media_type,duration,rating,actors,poster_path,file_path FROM media").unwrap();
    let rows = stmt.query_map([], |row| {
        let media_type_value: i64 = row.get(4)?;
        println!("row: id={} title={} duration={:?}", row.get::<_,i64>(0).unwrap(), row.get::<_,String>(1).unwrap(), row.get::<_,Option<String>>(5)?);
        Ok(())
    }).unwrap();
    let mut n = 0;
    for r in rows { if r.is_ok() { n+=1; } }
    println!("readable rows: {}", n);
}
