from dados import desnormalizar, normalizar


def avaliar(mlp, validacao, minimo_entrada, maximo_entrada, minimo_saida, maximo_saida):
    # // testa a MLP nos 7 pregoes que nao participaram do treinamento.
    resultados = []

    print("\nValidacao nos 7 pregoes mais recentes")
    print("Data       | Fechamento real | Previsao MLP | Erro absoluto")
    print("-" * 62)

    for amostra in validacao:
        entrada_norm = normalizar(amostra["entrada"], minimo_entrada, maximo_entrada)
        previsao_norm = mlp.prever(entrada_norm)
        previsao = desnormalizar(previsao_norm, minimo_saida, maximo_saida)
        erro = abs(amostra["esperado"] - previsao)

        resultados.append(
            {
                "data": amostra["data"],
                "real": amostra["esperado"],
                "previsao": previsao,
                "erro": erro,
            }
        )

        print(
            f"{amostra['data']} | R$ {amostra['esperado']:12.2f} | R$ {previsao:10.2f} | R$ {erro:11.2f}"
        )

    erro_medio = sum(resultado["erro"] for resultado in resultados) / len(resultados)
    print(f"\nErro absoluto medio na validacao: R$ {erro_medio:.2f}")

    return resultados
