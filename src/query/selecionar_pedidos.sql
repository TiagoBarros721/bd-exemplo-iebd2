SELECT pessoa.nome AS pess, produtos.nome AS prod, pedidos.status, pedidos.quantidade * produtos.preco AS total 
FROM pedidos
INNER JOIN pessoa ON pedidos.client = pessoa.id 
INNER JOIN produtos ON pedidos.produto = produtos.id
ORDER BY total;
