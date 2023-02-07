-- Contagens

SELECT COUNT(*) AS pedidos_totais FROM pedidos;
SELECT COUNT(*) AS produtos_totais FROM produtos;
SELECT COUNT(*) AS clientes_totais FROM pessoa;

SELECT COUNT(*) AS encomendas_entregues_totais FROM pedidos WHERE status = "Entregue";

-- Maximos
SELECT MAX(pedidos.quantidade) AS maior_quantidade_comprada FROM pedidos;
SELECT produtos.nome AS produto_mais_barato_nome, MIN(produtos.preco) AS produto_mais_barato_preco FROM produtos;

SELECT AVG(produtos.preco) AS media_preco_produtos FROM produtos;