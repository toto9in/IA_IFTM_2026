import os
import random
import sys

from cnn import CNNDigitos
from config import (
    CLASSES,
    EPOCAS,
    LIMITE_TESTE_POR_CLASSE,
    LIMITE_TREINO_POR_CLASSE,
    NOME_ARQUIVO_PESOS,
    QUANTIDADE_FILTROS,
    SEMENTE_ALEATORIA,
    TAMANHO_FILTRO,
    TAMANHO_IMAGEM,
    TAXA_APRENDIZADO,
)
from dados import carregar_dados_mnist
from imagem import carregar_imagem_para_rede
from utils import caminho_do_projeto


def mostrar_ajuda():
    # // mostra os dois modos disponiveis no programa.
    print("Uso:")
    print("  python main.py treinar")
    print("  python main.py prever caminho/da/imagem.png")


def modo_treinar():
    # // fixa a semente para o treino ficar reproduzivel.
    random.seed(SEMENTE_ALEATORIA)

    # // define onde estao os dados do MNIST e onde os pesos treinados serao salvos.
    pasta_mnist = caminho_do_projeto("data", "mnist")
    caminho_pesos = caminho_do_projeto(NOME_ARQUIVO_PESOS)

    try:
        # // carrega o MNIST filtrando apenas as classes escolhidas no trabalho.
        treino, teste, contagem_treino, contagem_teste = carregar_dados_mnist(
            pasta_mnist,
            CLASSES,
            LIMITE_TREINO_POR_CLASSE,
            LIMITE_TESTE_POR_CLASSE,
        )
    except (FileNotFoundError, ValueError) as erro:
        print(f"Erro ao carregar o MNIST: {erro}")
        return

    print("Classes usadas:", CLASSES)
    print("Amostras de treino por classe:", contagem_treino)
    print("Amostras de teste por classe:", contagem_teste)
    print(f"Total treino: {len(treino)}")
    print(f"Total teste: {len(teste)}")
    print(
        f"Arquitetura: entrada 28x28 -> {QUANTIDADE_FILTROS} filtros "
        f"{TAMANHO_FILTRO}x{TAMANHO_FILTRO} -> ReLU -> max pooling -> camada densa de {len(CLASSES)} saidas"
    )
    print(f"Taxa de aprendizado: {TAXA_APRENDIZADO}")
    print(f"Epocas: {EPOCAS}")

    # // cria a CNN, treina usando as imagens do MNIST e salva os pesos ao final.
    rede = CNNDigitos(
        TAMANHO_IMAGEM,
        QUANTIDADE_FILTROS,
        TAMANHO_FILTRO,
        CLASSES,
    )
    rede.treinar(treino, teste, TAXA_APRENDIZADO, EPOCAS)
    rede.salvar_pesos(caminho_pesos)

    print(f"\nPesos salvos em: {caminho_pesos}")


def modo_prever(caminho_imagem):
    caminho_pesos = caminho_do_projeto(NOME_ARQUIVO_PESOS)

    # // para prever uma nova imagem, primeiro precisamos carregar um modelo ja treinado.
    if not os.path.exists(caminho_pesos):
        print("Modelo treinado nao encontrado. Execute `python main.py treinar` primeiro.")
        return

    if not os.path.exists(caminho_imagem):
        print(f"Imagem nao encontrada: {caminho_imagem}")
        return

    rede = CNNDigitos.carregar_pesos(caminho_pesos)
    try:
        # // abre a imagem, converte para tons de cinza e ajusta para o tamanho da rede.
        imagem = carregar_imagem_para_rede(caminho_imagem, TAMANHO_IMAGEM)
    except OSError as erro:
        print(f"Erro ao abrir a imagem: {erro}")
        return

    # // a rede devolve a classe prevista e as probabilidades de cada numero.
    numero, probabilidades = rede.prever(imagem)

    print(f"Imagem analisada: {caminho_imagem}")
    print(f"Numero previsto: {numero}")
    print("Probabilidades por classe:")
    for classe, probabilidade in zip(CLASSES, probabilidades):
        print(f"  {classe}: {probabilidade * 100:.2f}%")


def main():
    # // o primeiro argumento escolhe se o usuario quer treinar ou prever.
    if len(sys.argv) < 2:
        mostrar_ajuda()
        return

    modo = sys.argv[1].lower()

    if modo == "treinar":
        modo_treinar()
        return

    if modo == "prever":
        if len(sys.argv) < 3:
            print("Informe o caminho da imagem.")
            print("Exemplo: python main.py prever data/exemplos_web/exemplo_1.png")
            return

        modo_prever(sys.argv[2])
        return

    mostrar_ajuda()


if __name__ == "__main__":
    main()
