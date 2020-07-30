use sqlite3;
use crate::cli;

pub fn get_db_connection() -> sqlite3::Connection {
    let mut db_pathbuf : std::path::PathBuf = match dirs::home_dir() {
        Some(path) => { path },
        None       => { std::path::PathBuf::from(".") },
    };
    db_pathbuf.push(cli::CONFIG_DIR);
    db_pathbuf.push("db.sqlite3");

    let db_path = db_pathbuf.as_path(); 
    let connection = sqlite3::open(db_path).unwrap();

    if ! db_path.exists() {
        create_tables(&connection);
    }
    
    connection
}

pub fn create_tables(connection: &sqlite3::Connection) {
    connection
        .execute(
            "
            DROP TABLE IF EXISTS piece_payload;
            CREATE TABLE piece_payload (msg_cid TEXT, piece_cid TEXT, payload_cid TEXT);
            ",
        )
        .unwrap();
}

pub fn insert_piece_and_payload_cids(connection: &sqlite3::Connection, msg_cid: &str, piece_cid:&str, payload_cid:&str) {
    // Insert
    connection
        .execute(format!(
            "INSERT INTO piece_payload (msg_cid, piece_cid, payload_cid) VALUES ('{}', '{}', '{}')",
            msg_cid,
            piece_cid,
            payload_cid
        ))
        .unwrap();
}