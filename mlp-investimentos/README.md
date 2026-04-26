# MLP Investimentos

Projeto academico em Python para treinar uma rede neural MLP simples e prever o valor de fechamento de uma acao brasileira a partir de uma data.

A implementacao usa `yfinance` apenas para baixar os dados historicos e implementa a rede neural manualmente, sem bibliotecas de machine learning.

## Como executar

Instale as dependencias:

```bash
pip install -r requirements.txt
```

Execute com o ticker padrao `BBAS3.SA`:

```bash
python main.py
```

Ou informe outro ticker aceito pelo Yahoo Finance:

```bash
python main.py PETR4.SA
```

## Ideia do modelo

- Entrada da rede: data do pregao convertida para numero e normalizada.
- Saida esperada: valor de fechamento da acao naquela data.
- Treino: dados anteriores aos 7 pregoes mais recentes, cobrindo aproximadamente os ultimos seis meses.
- Validacao: os 7 pregoes mais recentes.
- Arquitetura: `1 entrada -> 6 neuronios ocultos -> 1 saida`.
- Ativacao: sigmoid.
- Aprendizado: backpropagation implementado manualmente.

Depois de buscar os dados no `yfinance`, o programa imprime no terminal as datas e os valores de fechamento baixados. Ao final, tambem imprime uma tabela comparando previsao e valor real, alem de exibir um grafico com os fechamentos e as previsoes. Em ambientes sem janela grafica, o grafico e salvo como `previsao_<TICKER>.png`.

Depois do treino, o programa tambem abre um modo interativo. Nele, digite uma data e a rede ja treinada retorna a previsao de fechamento para essa data:

```text
Data para prever fechamento: 2026-04-27
Previsao de fechamento: R$ 23.10
```

Tambem e aceito o formato brasileiro:

```text
Data para prever fechamento: 27/04/2026
Previsao de fechamento: R$ 23.10
```

Para sair, pressione Enter sem digitar nada ou escreva `sair`.

## Organizacao dos arquivos

- `main.py`: coordena o fluxo principal do programa.
- `config.py`: guarda as configuracoes do experimento.
- `dados.py`: baixa dados, exibe dados baixados, cria amostras e normaliza valores.
- `mlp.py`: implementa a rede neural MLP e o backpropagation.
- `avaliacao.py`: valida a rede nos 7 pregoes mais recentes.
- `visualizacao.py`: gera o grafico dos fechamentos e previsoes.
- `interativo.py`: implementa o modo em que o usuario digita uma data.
