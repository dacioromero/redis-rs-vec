use redis::Commands;

fn main() {
    let mut conn = redis::Client::open("redis://127.0.0.1/")
        .unwrap()
        .get_connection()
        .unwrap();

    let nonexistent: Vec<Option<String>> = conn.get(vec!["nonexistent"]).unwrap();

    assert!(nonexistent.is_empty());

    let mut existent: Vec<Option<String>> = redis::cmd("MGET")
        .arg("existent")
        .query(&mut conn)
        .unwrap();

    assert!(existent.remove(0).unwrap() == "existent".to_owned());

    let _: redis::Value = conn.set(vec!["existent"], vec!["existent"]).unwrap();
    let mut existent: Vec<Option<String>> = conn.get(vec!["existent"]).unwrap();

    assert!(!existent.is_empty());
    assert!(existent.remove(0).unwrap() == "existent".to_owned());
}
