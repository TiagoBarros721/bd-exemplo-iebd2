use std::process::Command;
use std::{fs::File, io::Read};
use std::io::stdin;

use sqlite::Connection;

extern crate sqlite;

enum Status {
    ENTREGE,
    EM_PROCESSO,
    FALHADO
}

struct Pessoa {
    pub nome: String,
    pub apelido: String,
    pub morada: String
}

struct Pedido {
    pub cliente: String,
    pub produto: String,
    pub status: Status,
    pub quantidade: u16
}

struct Produtos {
    pub nome: String,
    pub preco: f32
}

fn main(){

    let connection: Connection = sqlite::open("src/pedidos.db").unwrap();
        
    let mut looping: bool = true;
    while looping {

        // Menu
        println!("Bem vindo à gestão de produtos com SQLite");
        println!("-- por tiago simões 12ºITM nº25 --");
        println!("\nEscolha uma das seguintes opções: ");
     
        println!("(0) Inserir pessoa");
        println!("(1) Ver pessoas");
        println!("(2) Inserir pedido");
        println!("(3) Ver pedidos");
        println!("(4) Entregar pedido");
        println!("(5) Dados do negócio");
        println!("(6) Fechar");

        let mut input_string = String::new();
        stdin().read_line(&mut input_string).ok().expect("Não foi possivel ler");

        let n: i32 = input_string.trim().parse().expect("A string inserida não é um número!");
        if n == 6 {looping = false;}

        // Tratar das funções
        match n {
            0 => {inserir_pessoa(&connection)},
            1 => {ver_pessoas(&connection)},
            2 => {inserir_pedido(&connection)},
            3 => ver_pedido(&connection),
            4 => {atualizar_encomenda(&connection)},
            5 => {estatisticas(&connection)},
            _ => {}
        }

        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    }

    println!("-----! Fim da execução !-----")
    
}

fn conseguir_query(file_name: &str) -> String {

    let mut file = File::open(format!("src/query/{}.sql", file_name)).expect("File not found");
    
    let mut data = String::new();
    file.read_to_string(&mut data).expect("Error while reading file");
   
    return data
}

fn executar_query(file_name: &str, conn: &Connection) {

    let mut file = File::open(format!("src/query/{}.sql", file_name)).expect("File not found");
    
    let mut data = String::new();
    file.read_to_string(&mut data).expect("Error while reading file");
   
    conn.execute(data).unwrap();
}

fn inserir_pessoa(conn: &Connection){

    println!("Escreva o nome da pessoa");
    let mut nome = String::new();
    stdin().read_line(&mut nome).ok().expect("Não foi possivel ler");

    println!("Escreva o apelido da pessoa");
    let mut apelido = String::new();
    stdin().read_line(&mut apelido).ok().expect("Não foi possivel ler");

    println!("Escreva a morada da pessoa");
    let mut morada = String::new();
    stdin().read_line(&mut morada).ok().expect("Não foi possivel ler");

    let query = format!("INSERT INTO pessoa(nome, apelido, morada) VALUES('{}', '{}', '{}')", nome.trim(), apelido.trim(), morada.trim());
    conn.execute(query).unwrap();

    println!("A pessoa {} {} foi inserida", nome.trim(), apelido.trim());
    let _ = Command::new("cmd.exe").arg("/c").arg("pause").status();
}

fn ver_pessoas(conn: &Connection) {

    let query = conseguir_query("seleciona_pessoas");
    let _ =conn.iterate(query, |pairs| {
        println!("{}: Nome: {} {} | Morada: {}", pairs.get(0).unwrap().1.unwrap(), pairs.get(1).unwrap().1.unwrap(), pairs.get(2).unwrap().1.unwrap(), pairs.get(3).unwrap().1.unwrap());
        true
    });

    let _ = Command::new("cmd.exe").arg("/c").arg("pause").status();
}

fn inserir_pedido(conn: &Connection){

    println!("Escreva o id do client");
    let mut cliente = String::new();
    stdin().read_line(&mut cliente).ok().expect("Não foi possivel ler");

    println!("Escreva o id do produto");
    let mut prod = String::new();
    stdin().read_line(&mut prod).ok().expect("Não foi possivel ler");

    println!("Escreva a quantidade a encomendar");
    let mut quantidade = String::new();
    stdin().read_line(&mut quantidade).ok().expect("Não foi possivel ler");

    let query = format!("INSERT INTO pedidos(client, produto, status, quantidade) VALUES({}, {}, 'Em Processo', {})", cliente.trim(), prod.trim(), quantidade.trim(), );
    conn.execute(query).unwrap();

    println!("A encomenda foi inserida");
    let _ = Command::new("cmd.exe").arg("/c").arg("pause").status();
}

fn ver_pedido(conn: &Connection) {

    let query = conseguir_query("selecionar_pedidos");
    let _ =conn.iterate(query, |pairs| {
        println!("Cliente: {} | Produto: {} | Estatuto da encomenda: {} | total a pagar: {}", pairs.get(0).unwrap().1.unwrap(), pairs.get(1).unwrap().1.unwrap(), pairs.get(2).unwrap().1.unwrap(), pairs.get(3).unwrap().1.unwrap());
        true
    });

    let _ = Command::new("cmd.exe").arg("/c").arg("pause").status();
}

fn estatisticas(conn: &Connection) {

    let query = conseguir_query("estatisticas");
    let _ =conn.iterate(query, |pairs| {
        for &(name, value) in pairs.iter() {
            println!("{} = {}", name, value.unwrap());
        }
        true
    }).unwrap();

    let _ = Command::new("cmd.exe").arg("/c").arg("pause").status();
}

fn atualizar_encomenda(conn: &Connection) {

    println!("Escreva o id da encomenda a atualizar");
    let mut encomenda = String::new();
    stdin().read_line(&mut encomenda).ok().expect("Não foi possivel ler");

    let status_list: Vec<&str> = vec!["Em Processo", "Entregue", "Falhado"];

    println!("Escolha um estado");
    for n in 0..status_list.len() {
        println!("({}) {}", n, status_list.get(n).unwrap());
    }

    let mut sid = String::new();
    stdin().read_line(&mut sid).ok().expect("Não foi possivel ler");

    let id: i32 = sid.trim().parse().expect("Status introduzido não é um numero");
    if id >= 3 || id < 0 {
        println!("Status introduzido está fora de alcance");
        let _ = Command::new("cmd.exe").arg("/c").arg("pause").status();
        return;
    }

    let query = !format("UPDATE pedidos SET status = '{}' WHERE id = {}", status_list.get(id), encomenda);

    let _ = Command::new("cmd.exe").arg("/c").arg("pause").status();
}