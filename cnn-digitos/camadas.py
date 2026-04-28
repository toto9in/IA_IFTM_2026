import math


def relu(valor):
    if valor > 0.0:
        return valor
    return 0.0


def derivada_relu(valor):
    if valor > 0.0:
        return 1.0
    return 0.0


def softmax(logits):
    maior = max(logits)
    exp_valores = [math.exp(valor - maior) for valor in logits]
    soma = sum(exp_valores)
    return [valor / soma for valor in exp_valores]


def zeros_matriz(linhas, colunas):
    return [[0.0 for _ in range(colunas)] for _ in range(linhas)]


def zeros_cubo(camadas, linhas, colunas):
    return [zeros_matriz(linhas, colunas) for _ in range(camadas)]
