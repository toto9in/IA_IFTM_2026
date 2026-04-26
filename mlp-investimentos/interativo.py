from datetime import datetime

from dados import desnormalizar, normalizar


def ler_data(texto):
    # // Aceita data no formato internacional ou brasileiro.
    for formato in ("%Y-%m-%d", "%d/%m/%Y"):
        try:
            return datetime.strptime(texto, formato).date()
        except ValueError:
            pass

    return None


def modo_interativo(mlp, minimo_entrada, maximo_entrada, minimo_saida, maximo_saida, data_inicial, data_final):
    # // Usa a rede ja treinada para prever o fechamento de uma data digitada.
    print("\nModo interativo")
    print("Digite uma data para prever o valor de fechamento da acao nesse dia.")
    print("Formatos aceitos: AAAA-MM-DD ou DD/MM/AAAA")
    print("Exemplo: 2026-04-27")
    print("Para encerrar, pressione Enter sem digitar nada ou escreva 'sair'.")

    while True:
        texto = input("\nData para prever fechamento: ").strip()

        if texto == "" or texto.lower() == "sair":
            print("Encerrando modo interativo.")
            break

        data_informada = ler_data(texto)
        if data_informada is None:
            print("Data invalida. Use AAAA-MM-DD, por exemplo 2026-04-27, ou DD/MM/AAAA.")
            continue

        entrada = (data_informada - data_inicial).days
        entrada_normalizada = normalizar(entrada, minimo_entrada, maximo_entrada)
        previsao_normalizada = mlp.prever(entrada_normalizada)
        previsao = desnormalizar(previsao_normalizada, minimo_saida, maximo_saida)

        if data_informada < data_inicial or data_informada > data_final:
            print("Aviso: data fora da janela baixada. A previsao e uma extrapolacao.")

        print(f"Data informada: {data_informada}")
        print(f"Previsao de fechamento: R$ {previsao:.2f}")
