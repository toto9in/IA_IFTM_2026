import matplotlib
import matplotlib.pyplot as plt


def plotar_resultados(datas, fechamentos, resultados, ticker):
    # // Desenha fechamentos reais e previsoes da MLP em um grafico.
    datas_validacao = [resultado["data"] for resultado in resultados]
    previsoes = [resultado["previsao"] for resultado in resultados]

    plt.figure(figsize=(11, 6))
    plt.plot(datas, fechamentos, label="Fechamento real", color="#1f77b4")
    plt.plot(datas_validacao, previsoes, marker="o", label="Previsao da MLP", color="#d62728")
    plt.title(f"Previsao de fechamento - {ticker}")
    plt.xlabel("Dia")
    plt.ylabel("Valor de fechamento (R$)")
    plt.grid(True, alpha=0.3)
    plt.legend()
    plt.tight_layout()

    # // Em ambiente sem janela grafica, salva o grafico em arquivo PNG.
    if "agg" in matplotlib.get_backend().lower():
        arquivo = f"previsao_{ticker.replace('.', '_')}.png"
        plt.savefig(arquivo, dpi=150)
        print(f"\nGrafico salvo em: {arquivo}")
    else:
        plt.show()
