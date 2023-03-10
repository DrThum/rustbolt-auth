use env_logger::Env;
use rusqlite::Connection;
use rustbolt_auth::{Realm, RealmType};
use std::sync::Arc;
use tokio::net::TcpListener;

mod embedded {
    use refinery::embed_migrations;
    embed_migrations!("../sql_migrations/auth");
}

#[tokio::main]
async fn main() {
    // Setup logging
    env_logger::Builder::from_env(Env::default().default_filter_or("debug")).init();

    // Execute database migrations
    let mut conn = Connection::open("./data/databases/auth.db").unwrap();
    embedded::migrations::runner().run(&mut conn).unwrap();

    // Load realms from database
    let realms = load_realms_from_db(&mut conn);

    // Bind the listener to the address
    let listener = TcpListener::bind("127.0.0.1:3724").await.unwrap();

    loop {
        // The second item contains the IP and port of the new connection
        let (socket, _) = listener.accept().await.unwrap();

        // Spawn a new task for each inbound socket
        let realms_copy = Arc::clone(&realms);
        tokio::spawn(async move {
            rustbolt_auth::process(socket, realms_copy)
                .await
                .expect("Parse error during auth sequence");
        });
    }
}

fn load_realms_from_db(conn: &mut Connection) -> Arc<Vec<Realm>> {
    let mut stmt = conn.prepare("SELECT id, realm_type, is_locked, flags, name, address, population, category FROM realms").unwrap();
    let realm_iter = stmt
        .query_map([], |row| {
            let realm_type: i32 = row.get("realm_type")?;
            let realm_type = RealmType::try_from(realm_type).unwrap();
            let locked: i32 = row.get("is_locked")?;
            let locked = locked > 0;
            let realm_name: String = row.get("name")?;
            let address: String = row.get("address")?;

            Ok(Realm {
                _realm_type: realm_type,
                _locked: locked,
                _realm_flags: row.get("flags")?,
                _realm_name: realm_name.try_into().unwrap(),
                _address_port: address.try_into().unwrap(),
                _population: row.get("population")?,
                _num_chars: 1,
                _realm_category: row.get("category")?,
                _realm_id: row.get("id")?,
            })
        })
        .unwrap();

    Arc::new(realm_iter.map(|r| r.unwrap()).collect())
}
