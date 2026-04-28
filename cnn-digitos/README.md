# CNN Digitos

Projeto academico em Python para classificar imagens dos numeros `0` a `5` com uma Rede Neural Convolucional feita manualmente, sem bibliotecas de machine learning.

A CNN usa:

- convolucao manual
- funcao de ativacao `ReLU`
- max pooling
- camada densa final com `softmax`

O programa tem dois modos:

- `treinar`: treina a rede com o MNIST filtrado para as classes `0, 1, 2, 3, 4, 5`
- `prever`: le uma imagem a partir de um diretorio e informa qual numero foi reconhecido

## Requisitos

Instale a unica dependencia usada para abrir e ajustar imagens:

```bash
pip install -r requirements.txt
```

## Estrutura esperada do MNIST

Coloque os arquivos do MNIST dentro de `data/mnist/`.

Arquivos aceitos:

```text
train-images-idx3-ubyte
train-labels-idx1-ubyte
t10k-images-idx3-ubyte
t10k-labels-idx1-ubyte
```

Tambem funciona se os arquivos estiverem compactados com extensao `.gz`.

## Como executar

Treinar a rede:

```bash
python main.py treinar
```

Prever uma imagem:

```bash
python main.py prever data/exemplos_web/exemplo_1.png
```

## Sobre as classes do trabalho

Esta versao da rede classifica apenas:

```text
0, 1, 2, 3, 4, 5
```

O treino usa varias amostras do MNIST para cada uma dessas classes.

## Sobre as 5 imagens da web

A pasta `data/exemplos_web/` foi criada para voce colocar 5 imagens de numeros baixadas da web e testar o modo `prever`.

Sugestao:

- `exemplo_0.png`
- `exemplo_1.png`
- `exemplo_2.png`
- `exemplo_3.png`
- `exemplo_4.png`

Se a imagem vier com fundo branco e digito escuro, o programa tenta inverter automaticamente para ficar parecida com o padrao do MNIST.

## Observacoes importantes

- O codigo foi escrito para ser simples e didatico.
- A rede foi feita sem `numpy`, `tensorflow`, `keras`, `pytorch` ou bibliotecas de machine learning.
- Como o treino e todo manual, os limites de amostras por classe foram reduzidos no `config.py` para deixar a execucao viavel.
- Depois do treino, os pesos sao salvos em `pesos.json`.
