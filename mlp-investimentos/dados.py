from dataclasses import dataclass
from datetime import date, timedelta
from typing import Any

import yfinance as yf

from config import DIAS_BUSCA, DIAS_VALIDACAO


@dataclass
class DadosPreparados:
    treino: list[Any]
    validacao: list[Any]
    treino_normalizado: list[tuple[float, float]]
    minimo_entrada: float
    maximo_entrada: float
    minimo_saida: float
    maximo_saida: float


def baixar_fechamentos(ticker):
    # // busca no yfinance as datas dos pregoes e o fechamento de cada data.
    inicio = date.today() - timedelta(days=DIAS_BUSCA)
    dados = yf.download(
        ticker, start=inicio.isoformat(), progress=False, auto_adjust=False
    )

    if dados is None or dados.empty or "Close" not in dados:
        raise RuntimeError(f"Nao foram encontrados dados para o ticker {ticker}.")

    serie = dados["Close"].dropna()

    # // Algumas versoes do yfinance retornam DataFrame mesmo para um ticker unico.
    if hasattr(serie, "columns"):
        serie = serie.iloc[:, 0]

    datas = [indice.date() for indice in serie.index]
    fechamentos = [float(valor) for valor in serie.values]

    if len(fechamentos) < 40:
        raise RuntimeError("Poucos dados encontrados para treinar e validar a rede.")

    return datas, fechamentos


def exibir_dados_baixados(datas, fechamentos):
    # // mostra no terminal os dados que vieram do yfinance.
    print("\nDados baixados do yfinance")
    print("Data       | Fechamento")
    print("-" * 27)

    for data_pregao, fechamento in zip(datas, fechamentos):
        print(f"{data_pregao} | R$ {fechamento:9.2f}")


def criar_amostras(datas, fechamentos):
    # // cria amostras no formato: data numerica -> fechamento real.
    amostras = []
    data_inicial = datas[0]

    for i in range(len(fechamentos)):
        amostras.append(
            {
                "data": datas[i],
                "entrada": (datas[i] - data_inicial).days,
                "esperado": fechamentos[i],
            }
        )

    return amostras


def normalizar(valor, minimo, maximo):
    # // converte um valor do intervalo [minimo, maximo] para [0, 1].
    if maximo == minimo:
        return 0.5
    return (valor - minimo) / (maximo - minimo)


def desnormalizar(valor, minimo, maximo):
    # // converte um valor normalizado de volta para a escala original.
    return valor * (maximo - minimo) + minimo


def preparar_dados(amostras):
    # // separa dados antigos para treino e os 7 ultimos pregoes para validacao.
    validacao = amostras[-DIAS_VALIDACAO:]
    treino = amostras[:-DIAS_VALIDACAO]

    entradas_treino = [amostra["entrada"] for amostra in treino]
    saidas_treino = [amostra["esperado"] for amostra in treino]

    minimo_entrada = min(entradas_treino)
    maximo_entrada = max(entradas_treino)
    minimo_saida = min(saidas_treino)
    maximo_saida = max(saidas_treino)

    treino_normalizado = []
    for amostra in treino:
        entrada_norm = normalizar(amostra["entrada"], minimo_entrada, maximo_entrada)
        saida_norm = normalizar(amostra["esperado"], minimo_saida, maximo_saida)
        treino_normalizado.append((entrada_norm, saida_norm))

    return DadosPreparados(
        treino=treino,
        validacao=validacao,
        treino_normalizado=treino_normalizado,
        minimo_entrada=minimo_entrada,
        maximo_entrada=maximo_entrada,
        minimo_saida=minimo_saida,
        maximo_saida=maximo_saida,
    )
