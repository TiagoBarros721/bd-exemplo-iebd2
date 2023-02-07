use rusqlite;

fn main(){

  let conn = rusqlite::Connection::open("pedidos.db")?;
  println!("Hello, World");
}
