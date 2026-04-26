import random
import sys

from avaliacao import avaliar
from config import EPOCAS, NEURONIOS_OCULTOS, TAXA_APRENDIZADO, TICKER_PADRAO
from dados import (
    baixar_fechamentos,
    criar_amostras,
    exibir_dados_baixados,
    preparar_dados,
)
from interativo import modo_interativo
from mlp import MLP
from visualizacao import plotar_resultados


def main():
    random.seed(42)

    ticker = sys.argv[1] if len(sys.argv) > 1 else TICKER_PADRAO
    print(f"Ticker usado: {ticker}")

    # baixa os dados historicos da acao.
    datas, fechamentos = baixar_fechamentos(ticker)

    # mostra no terminal os dados baixados.
    exibir_dados_baixados(datas, fechamentos)

    # // converte datas e fechamentos em amostras numericas.
    amostras = criar_amostras(datas, fechamentos)

    # // separa treino/validacao e normaliza os valores.
    dados = preparar_dados(amostras)

    # // mostra a configuracao da rede e dos dados.
    print(f"Pregoes baixados: {len(fechamentos)}")
    print(f"Amostras de treino: {len(dados.treino)}")
    print(f"Amostras de validacao: {len(dados.validacao)}")
    print("Entrada da rede: data do pregao convertida para numero")
    print(f"Arquitetura: 1 entrada -> {NEURONIOS_OCULTOS} neuronios ocultos -> 1 saida")
    print(f"Taxa de aprendizado: {TAXA_APRENDIZADO}")
    print(f"Epocas: {EPOCAS}\n")

    # // cria e treina a MLP.
    mlp = MLP(NEURONIOS_OCULTOS)
    mlp.treinar(dados.treino_normalizado, TAXA_APRENDIZADO, EPOCAS)

    # // valida nos 7 pregoes mais recentes.
    resultados = avaliar(
        mlp,
        dados.validacao,
        dados.minimo_entrada,
        dados.maximo_entrada,
        dados.minimo_saida,
        dados.maximo_saida,
    )

    # // gera o grafico dos fechamentos e previsoes.
    plotar_resultados(datas, fechamentos, resultados, ticker)

    # // modo interativo: digitar uma data para prever o fechamento.
    modo_interativo(
        mlp,
        dados.minimo_entrada,
        dados.maximo_entrada,
        dados.minimo_saida,
        dados.maximo_saida,
        datas[0],
        datas[-1],
    )


if __name__ == "__main__":
    main()
