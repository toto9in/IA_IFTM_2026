import math
import random


def sigmoid(x):
    # // Funcao de ativacao: transforma qualquer valor real em numero entre 0 e 1.
    return 1.0 / (1.0 + math.exp(-x))


def derivada_sigmoid(saida):
    # // Derivada da sigmoid usada no backpropagation.
    return saida * (1.0 - saida)


class MLP:
    # // rede neural MLP: 1 entrada -> camada oculta -> 1 saida.
    def __init__(self, neuronios_ocultos):
        self.neuronios_ocultos = neuronios_ocultos

        # // como existe apenas uma entrada, cada neuronio oculto tem um peso.
        self.pesos_ocultos = [
            random.uniform(-0.5, 0.5) for _ in range(neuronios_ocultos)
        ]
        self.bias_ocultos = [
            random.uniform(-0.5, 0.5) for _ in range(neuronios_ocultos)
        ]

        # // cada neuronio oculto se liga ao unico neuronio de saida.
        self.pesos_saida = [random.uniform(-0.5, 0.5) for _ in range(neuronios_ocultos)]
        self.bias_saida = random.uniform(-0.5, 0.5)

    def forward(self, entrada):
        # // CAMADA OCULTA: cada neuronio recebe a entrada, multiplica pelo seu peso,
        # // soma o bias (ajuste fixo), e passa pelo sigmoid para virar um valor entre 0 e 1.
        saidas_ocultas = []

        for i in range(self.neuronios_ocultos):
            soma = entrada * self.pesos_ocultos[i] + self.bias_ocultos[i]
            saidas_ocultas.append(sigmoid(soma))

        # // NEURONIO DE SAIDA: recebe as saidas de todos os neuronios ocultos,
        # // cada uma multiplicada pelo seu peso, soma o bias da saida.
        soma_saida = self.bias_saida
        for i in range(self.neuronios_ocultos):
            soma_saida += saidas_ocultas[i] * self.pesos_saida[i]

        # // passa pelo sigmoid para garantir que a saida final fica entre 0 e 1
        # // (mesma escala dos dados normalizados).
        saida = sigmoid(soma_saida)
        return saidas_ocultas, saida

    def prever(self, entrada):
        # // Retorna somente a saida final da rede.
        _, saida = self.forward(entrada)
        return saida

    def treinar(self, amostras, taxa_aprendizado, epocas):
        for epoca in range(1, epocas + 1):
            erro_total = 0.0

            # // embaralha para a rede nao memorizar a ordem das amostras.
            random.shuffle(amostras)

            for entrada, esperado in amostras:
                # // forward pass: a rede faz uma previsao com os pesos atuais.
                saidas_ocultas, saida = self.forward(entrada)

                # // calcula o erro: diferenca entre o que era esperado e o que saiu.
                erro = esperado - saida

                # // acumula o erro quadratico para monitorar o treino.
                erro_total += erro**2

                # // delta da saida: o quanto o neuronio de saida errou,
                # // multiplicado pela derivada da sigmoid
                delta_saida = erro * derivada_sigmoid(saida)

                # // backpropagation: propaga o erro de volta para a camada oculta.
                # // cada neuronio oculto recebe uma parcela do erro proporcional ao seu peso.
                deltas_ocultos = []
                for i in range(self.neuronios_ocultos):
                    erro_oculto = delta_saida * self.pesos_saida[i]
                    deltas_ocultos.append(
                        erro_oculto * derivada_sigmoid(saidas_ocultas[i])
                    )

                # // ajusta pesos da camada de saida.
                # // pesos que contribuiram mais para o erro sao corrigidos mais.
                for i in range(self.neuronios_ocultos):
                    self.pesos_saida[i] += (
                        taxa_aprendizado * delta_saida * saidas_ocultas[i]
                    )
                self.bias_saida += taxa_aprendizado * delta_saida

                # // ajusta pesos da camada oculta.
                for i in range(self.neuronios_ocultos):
                    self.pesos_ocultos[i] += (
                        taxa_aprendizado * deltas_ocultos[i] * entrada
                    )
                    self.bias_ocultos[i] += taxa_aprendizado * deltas_ocultos[i]

            # // imprime o erro medio a cada 500 epocas para acompanhar a convergencia.
            if epoca == 1 or epoca % 500 == 0:
                erro_medio = erro_total / len(amostras)
                print(f"Epoca {epoca:4d} | erro medio: {erro_medio:.8f}")
