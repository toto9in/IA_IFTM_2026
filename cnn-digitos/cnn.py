import math
import random

from camadas import derivada_relu, relu, softmax, zeros_cubo, zeros_matriz
from utils import carregar_json, salvar_json


class CNNDigitos:
    # // DIAGRAMA GERAL DA REDE
    # //
    # // imagem 28x28
    # //      |
    # //      v
    # // convolucao com 4 filtros 3x3
    # //      |
    # //      v
    # // mapas de caracteristicas 26x26
    # //      |
    # //      v
    # // ReLU
    # //      |
    # //      v
    # // max pooling 2x2
    # //      |
    # //      v
    # // mapas reduzidos 13x13
    # //      |
    # //      v
    # // flatten
    # //      |
    # //      v
    # // vetor unico com todos os valores
    # //      |
    # //      v
    # // camada densa final
    # //      |
    # //      v
    # // logits das classes 0, 1, 2, 3, 4, 5
    # //      |
    # //      v
    # // softmax -> probabilidades finais
    # //
    # // RESUMO DO TREINO
    # //
    # // 1. a imagem entra na rede
    # // 2. a rede calcula uma previsao
    # // 3. comparamos a previsao com o rotulo correto
    # // 4. calculamos o erro
    # // 5. o erro volta da saida para os filtros
    # // 6. os pesos sao ajustados para errar menos na proxima vez
    def __init__(self, tamanho_imagem, quantidade_filtros, tamanho_filtro, classes):
        self.tamanho_imagem = tamanho_imagem
        self.quantidade_filtros = quantidade_filtros
        self.tamanho_filtro = tamanho_filtro
        self.classes = classes
        self.quantidade_classes = len(classes)

        # // calcula o tamanho das saidas internas depois da convolucao e do pooling.
        self.tamanho_convolucao = tamanho_imagem - tamanho_filtro + 1
        self.tamanho_pooling = self.tamanho_convolucao // 2
        self.tamanho_flatten = (
            quantidade_filtros * self.tamanho_pooling * self.tamanho_pooling
        )

        # // cria os filtros da camada convolucional com valores pequenos aleatorios.
        self.filtros = []
        self.bias_filtros = []
        for _ in range(quantidade_filtros):
            filtro = []
            for _linha in range(tamanho_filtro):
                linha = []
                for _coluna in range(tamanho_filtro):
                    linha.append(random.uniform(-0.1, 0.1))
                filtro.append(linha)
            self.filtros.append(filtro)
            self.bias_filtros.append(random.uniform(-0.1, 0.1))

        # // cria os pesos da camada de saida, que recebe o vetor achatado do pooling.
        self.pesos_saida = []
        for _ in range(self.quantidade_classes):
            linha = []
            for _ in range(self.tamanho_flatten):
                linha.append(random.uniform(-0.1, 0.1))
            self.pesos_saida.append(linha)

        self.bias_saida = [
            random.uniform(-0.1, 0.1) for _ in range(self.quantidade_classes)
        ]

    def forward(self, imagem):
        # // ETAPA 1: a imagem passa primeiro pela convolucao.
        # // aqui cada filtro tenta encontrar algum padrao util, como bordas e tracos.
        # // conv_pre guarda as somas antes da ReLU e conv_pos guarda a ativacao depois da ReLU.
        conv_pre = zeros_cubo(
            self.quantidade_filtros,
            self.tamanho_convolucao,
            self.tamanho_convolucao,
        )
        conv_pos = zeros_cubo(
            self.quantidade_filtros,
            self.tamanho_convolucao,
            self.tamanho_convolucao,
        )

        # // camada convolucional: cada filtro percorre a imagem e produz um mapa de caracteristicas.
        for indice_filtro in range(self.quantidade_filtros):
            for linha in range(self.tamanho_convolucao):
                for coluna in range(self.tamanho_convolucao):
                    soma = self.bias_filtros[indice_filtro]
                    for desl_linha in range(self.tamanho_filtro):
                        for desl_coluna in range(self.tamanho_filtro):
                            soma += (
                                imagem[linha + desl_linha][coluna + desl_coluna]
                                * self.filtros[indice_filtro][desl_linha][desl_coluna]
                            )

                    conv_pre[indice_filtro][linha][coluna] = soma
                    conv_pos[indice_filtro][linha][coluna] = relu(soma)

        # // ETAPA 2: depois da convolucao vem o pooling.
        # // ele resume pequenas regioes e reduz a quantidade de dados.
        # // max pooling: reduz o tamanho do mapa e mantem apenas os valores mais fortes.
        pooling = zeros_cubo(
            self.quantidade_filtros,
            self.tamanho_pooling,
            self.tamanho_pooling,
        )
        posicoes_maximas = []

        for indice_filtro in range(self.quantidade_filtros):
            posicoes_filtro = []
            for linha in range(self.tamanho_pooling):
                posicoes_linha = []
                for coluna in range(self.tamanho_pooling):
                    base_linha = linha * 2
                    base_coluna = coluna * 2
                    maior_valor = -1.0
                    maior_posicao = (base_linha, base_coluna)

                    for desl_linha in range(2):
                        for desl_coluna in range(2):
                            valor = conv_pos[indice_filtro][base_linha + desl_linha][
                                base_coluna + desl_coluna
                            ]
                            if valor > maior_valor:
                                maior_valor = valor
                                maior_posicao = (
                                    base_linha + desl_linha,
                                    base_coluna + desl_coluna,
                                )

                    pooling[indice_filtro][linha][coluna] = maior_valor
                    posicoes_linha.append(maior_posicao)

                posicoes_filtro.append(posicoes_linha)
            posicoes_maximas.append(posicoes_filtro)

        # // ETAPA 3: junta os mapas reduzidos em um vetor linear.
        # // flatten: transforma todos os mapas reduzidos em um unico vetor.
        flatten = []
        for indice_filtro in range(self.quantidade_filtros):
            for linha in range(self.tamanho_pooling):
                for coluna in range(self.tamanho_pooling):
                    flatten.append(pooling[indice_filtro][linha][coluna])

        # // ETAPA 4: a camada densa mistura todas essas informacoes para decidir a classe.
        # // camada densa final: combina o vetor achatado e gera um logit para cada classe.
        logits = []
        for indice_classe in range(self.quantidade_classes):
            soma = self.bias_saida[indice_classe]
            for indice_valor, valor in enumerate(flatten):
                soma += valor * self.pesos_saida[indice_classe][indice_valor]
            logits.append(soma)

        # // ETAPA 5: o softmax transforma os logits em probabilidades.
        probabilidades = softmax(logits)

        cache = {
            "imagem": imagem,
            "conv_pre": conv_pre,
            "conv_pos": conv_pos,
            "pooling": pooling,
            "flatten": flatten,
            "posicoes_maximas": posicoes_maximas,
            "logits": logits,
            "probabilidades": probabilidades,
        }
        return cache

    def prever(self, imagem):
        # // converte a saida da rede no numero real da classe, como 0, 1, 2...
        cache = self.forward(imagem)
        probabilidades = cache["probabilidades"]
        indice = probabilidades.index(max(probabilidades))
        return self.classes[indice], probabilidades

    def treinar(self, amostras_treino, amostras_teste, taxa_aprendizado, epocas):
        for epoca in range(1, epocas + 1):
            # // embaralha as amostras para a rede nao aprender a ordem do arquivo.
            random.shuffle(amostras_treino)
            perda_total = 0.0
            acertos = 0

            for imagem, rotulo_correto in amostras_treino:
                # // ETAPA 1 DO TREINO: a imagem percorre a rede no sentido normal.
                # // forward pass: a rede gera as probabilidades para a imagem atual.
                cache = self.forward(imagem)
                probabilidades = cache["probabilidades"]
                flatten = cache["flatten"]

                # // ETAPA 2 DO TREINO: medimos o erro comparando a saida com o rotulo esperado.
                # // usa log-loss para medir o quanto a rede errou na classe correta.
                prob_correta = max(probabilidades[rotulo_correto], 1e-10)
                perda_total += -math.log(prob_correta)

                indice_previsto = probabilidades.index(max(probabilidades))
                if indice_previsto == rotulo_correto:
                    acertos += 1

                # // ETAPA 3 DO TREINO: criamos o gradiente da camada de saida.
                # // gradiente da combinacao softmax + entropia cruzada.
                grad_logits = probabilidades[:]
                grad_logits[rotulo_correto] -= 1.0

                # // copia os pesos da saida antes da atualizacao para usar na retropropagacao.
                pesos_saida_antes = [linha[:] for linha in self.pesos_saida]

                # // ETAPA 4 DO TREINO: ajustamos primeiro a camada densa final.
                # // ajusta os pesos da camada densa final.
                for indice_classe in range(self.quantidade_classes):
                    for indice_valor, valor in enumerate(flatten):
                        self.pesos_saida[indice_classe][indice_valor] -= (
                            taxa_aprendizado * grad_logits[indice_classe] * valor
                        )
                    self.bias_saida[indice_classe] -= (
                        taxa_aprendizado * grad_logits[indice_classe]
                    )

                # // ETAPA 5 DO TREINO: o erro volta para o vetor flatten.
                # // devolve o erro da camada de saida para o vetor achatado.
                grad_flatten = [0.0 for _ in range(self.tamanho_flatten)]
                for indice_valor in range(self.tamanho_flatten):
                    soma = 0.0
                    for indice_classe in range(self.quantidade_classes):
                        soma += (
                            grad_logits[indice_classe]
                            * pesos_saida_antes[indice_classe][indice_valor]
                        )
                    grad_flatten[indice_valor] = soma

                # // ETAPA 6 DO TREINO: o vetor de gradientes volta ao formato dos mapas pooled.
                # // reorganiza o gradiente do vetor achatado no formato dos mapas apos o pooling.
                grad_pooling = zeros_cubo(
                    self.quantidade_filtros,
                    self.tamanho_pooling,
                    self.tamanho_pooling,
                )

                indice_flatten = 0
                for indice_filtro in range(self.quantidade_filtros):
                    for linha in range(self.tamanho_pooling):
                        for coluna in range(self.tamanho_pooling):
                            grad_pooling[indice_filtro][linha][coluna] = grad_flatten[
                                indice_flatten
                            ]
                            indice_flatten += 1

                # // ETAPA 7 DO TREINO: no pooling, o erro retorna apenas para o maior valor escolhido.
                # // no max pooling, o gradiente volta apenas para a posicao que tinha o maior valor.
                grad_conv_pos = zeros_cubo(
                    self.quantidade_filtros,
                    self.tamanho_convolucao,
                    self.tamanho_convolucao,
                )

                for indice_filtro in range(self.quantidade_filtros):
                    for linha in range(self.tamanho_pooling):
                        for coluna in range(self.tamanho_pooling):
                            pos_linha, pos_coluna = cache["posicoes_maximas"][
                                indice_filtro
                            ][linha][coluna]
                            grad_conv_pos[indice_filtro][pos_linha][pos_coluna] += (
                                grad_pooling[indice_filtro][linha][coluna]
                            )

                # // ETAPA 8 DO TREINO: a derivada da ReLU bloqueia gradientes em valores negativos.
                # // aplica a derivada da ReLU para descobrir onde o gradiente continua.
                grad_conv_pre = zeros_cubo(
                    self.quantidade_filtros,
                    self.tamanho_convolucao,
                    self.tamanho_convolucao,
                )

                for indice_filtro in range(self.quantidade_filtros):
                    for linha in range(self.tamanho_convolucao):
                        for coluna in range(self.tamanho_convolucao):
                            grad_conv_pre[indice_filtro][linha][coluna] = grad_conv_pos[
                                indice_filtro
                            ][linha][coluna] * derivada_relu(
                                cache["conv_pre"][indice_filtro][linha][coluna]
                            )

                # // ETAPA 9 DO TREINO: corrigimos os filtros que olharam para a imagem.
                # // calcula e aplica a correcao dos filtros convolucionais e dos biases.
                for indice_filtro in range(self.quantidade_filtros):
                    grad_filtro = zeros_matriz(self.tamanho_filtro, self.tamanho_filtro)
                    grad_bias = 0.0

                    for linha in range(self.tamanho_convolucao):
                        for coluna in range(self.tamanho_convolucao):
                            gradiente_local = grad_conv_pre[indice_filtro][linha][
                                coluna
                            ]
                            grad_bias += gradiente_local

                            for desl_linha in range(self.tamanho_filtro):
                                for desl_coluna in range(self.tamanho_filtro):
                                    grad_filtro[desl_linha][desl_coluna] += (
                                        cache["imagem"][linha + desl_linha][
                                            coluna + desl_coluna
                                        ]
                                        * gradiente_local
                                    )

                    for desl_linha in range(self.tamanho_filtro):
                        for desl_coluna in range(self.tamanho_filtro):
                            self.filtros[indice_filtro][desl_linha][desl_coluna] -= (
                                taxa_aprendizado * grad_filtro[desl_linha][desl_coluna]
                            )

                    self.bias_filtros[indice_filtro] -= taxa_aprendizado * grad_bias

            # // ao fim de cada epoca, mostra erro medio e acuracia em treino e teste.
            perda_media = perda_total / len(amostras_treino)
            acuracia_treino = acertos / len(amostras_treino)
            acuracia_teste = self.avaliar(amostras_teste)

            print(
                f"Epoca {epoca:2d} | perda media: {perda_media:.4f} | "
                f"acuracia treino: {acuracia_treino * 100:.2f}% | "
                f"acuracia teste: {acuracia_teste * 100:.2f}%"
            )

    def avaliar(self, amostras):
        if not amostras:
            return 0.0

        # // mede quantas imagens a rede acertou sem atualizar pesos.
        acertos = 0
        for imagem, rotulo_correto in amostras:
            indice_previsto, _ = self.prever_indice(imagem)
            if indice_previsto == rotulo_correto:
                acertos += 1

        return acertos / len(amostras)

    def prever_indice(self, imagem):
        cache = self.forward(imagem)
        probabilidades = cache["probabilidades"]
        indice = probabilidades.index(max(probabilidades))
        return indice, probabilidades

    def salvar_pesos(self, caminho):
        # // salva os parametros treinados em JSON para reutilizar no modo prever.
        salvar_json(
            caminho,
            {
                "tamanho_imagem": self.tamanho_imagem,
                "quantidade_filtros": self.quantidade_filtros,
                "tamanho_filtro": self.tamanho_filtro,
                "classes": self.classes,
                "filtros": self.filtros,
                "bias_filtros": self.bias_filtros,
                "pesos_saida": self.pesos_saida,
                "bias_saida": self.bias_saida,
            },
        )

    @classmethod
    def carregar_pesos(cls, caminho):
        # // recria o modelo e recoloca os pesos que foram salvos no treino.
        dados = carregar_json(caminho)
        modelo = cls(
            dados["tamanho_imagem"],
            dados["quantidade_filtros"],
            dados["tamanho_filtro"],
            dados["classes"],
        )
        modelo.filtros = dados["filtros"]
        modelo.bias_filtros = dados["bias_filtros"]
        modelo.pesos_saida = dados["pesos_saida"]
        modelo.bias_saida = dados["bias_saida"]
        return modelo
