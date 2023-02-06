CREATE TABLE pedidos (
    id INTEGER PRIMARY KEY,
    client INTEGER, 
    produto INTEGER,
    status VARCHAR(100),
    quantidade INTEGER
);