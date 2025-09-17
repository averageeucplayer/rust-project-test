use log::info;
use refinery::Migration;
use refinery_core::traits::sync::migrate;
use rusqlite::Connection;

// without "IF NOT EXISTS"
// it explodes on existing database
// mod embedded {
//     use refinery::embed_migrations;
//     embed_migrations!("./migrations_pure");
// }

// with "IF NOT EXISTS"
mod embedded {
    use refinery::embed_migrations;
    embed_migrations!("./migrations_tweaks");
}

fn main() {

    let mut connection = Connection::open("encounters.db").unwrap();
    let runner = embedded::migrations::runner();

    runner.run(&mut connection).unwrap();
}