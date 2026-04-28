import gzip
import os
import struct


def abrir_arquivo_binario(caminho):
    # // aceita tanto os arquivos puros do MNIST quanto a versao compactada em .gz.
    if caminho.endswith(".gz"):
        return gzip.open(caminho, "rb")
    return open(caminho, "rb")


def ler_rotulos_idx(caminho):
    with abrir_arquivo_binario(caminho) as arquivo:
        # // o cabecalho informa o tipo do arquivo e quantos rotulos existem.
        magic, quantidade = struct.unpack(">II", arquivo.read(8))
        if magic != 2049:
            raise ValueError(f"Arquivo de rotulos invalido: {caminho}")
        return list(arquivo.read(quantidade))


def ler_imagens_idx(caminho):
    with abrir_arquivo_binario(caminho) as arquivo:
        # // o arquivo de imagens informa quantidade, linhas e colunas de cada imagem.
        magic, quantidade, linhas, colunas = struct.unpack(">IIII", arquivo.read(16))
        if magic != 2051:
            raise ValueError(f"Arquivo de imagens invalido: {caminho}")

        imagens = []
        tamanho = linhas * colunas

        for _ in range(quantidade):
            bruto = arquivo.read(tamanho)
            imagem = []
            indice = 0

            # // converte o vetor bruto de bytes em uma matriz 28x28 com valores entre 0 e 1.
            for _linha in range(linhas):
                linha = []
                for _coluna in range(colunas):
                    pixel = bruto[indice] / 255.0
                    linha.append(pixel)
                    indice += 1
                imagem.append(linha)

            imagens.append(imagem)

        return imagens


def localizar_arquivo(base_dir, candidatos):
    # // procura o primeiro nome valido dentro da pasta informada.
    for nome in candidatos:
        caminho = os.path.join(base_dir, nome)
        if os.path.exists(caminho):
            return caminho
    return None


def carregar_amostras_filtradas(caminho_imagens, caminho_rotulos, classes_desejadas, limite_por_classe):
    imagens = ler_imagens_idx(caminho_imagens)
    rotulos = ler_rotulos_idx(caminho_rotulos)

    # // cria um mapeamento da classe real para o indice usado na saida da rede.
    contagem_por_classe = {classe: 0 for classe in classes_desejadas}
    classe_para_indice = {classe: indice for indice, classe in enumerate(classes_desejadas)}
    amostras = []

    for imagem, rotulo in zip(imagens, rotulos):
        if rotulo not in classe_para_indice:
            continue

        if contagem_por_classe[rotulo] >= limite_por_classe:
            continue

        # // guarda a imagem junto do indice da classe correspondente.
        amostras.append((imagem, classe_para_indice[rotulo]))
        contagem_por_classe[rotulo] += 1

        # // para de ler quando todas as classes ja atingiram o limite definido.
        if all(contagem >= limite_por_classe for contagem in contagem_por_classe.values()):
            break

    return amostras, contagem_por_classe


def carregar_dados_mnist(base_dir, classes_desejadas, limite_treino_por_classe, limite_teste_por_classe):
    # // localiza os quatro arquivos principais: imagens e rotulos de treino e teste.
    caminho_treino_imagens = localizar_arquivo(
        base_dir,
        ["train-images-idx3-ubyte", "train-images-idx3-ubyte.gz"],
    )
    caminho_treino_rotulos = localizar_arquivo(
        base_dir,
        ["train-labels-idx1-ubyte", "train-labels-idx1-ubyte.gz"],
    )
    caminho_teste_imagens = localizar_arquivo(
        base_dir,
        ["t10k-images-idx3-ubyte", "t10k-images-idx3-ubyte.gz"],
    )
    caminho_teste_rotulos = localizar_arquivo(
        base_dir,
        ["t10k-labels-idx1-ubyte", "t10k-labels-idx1-ubyte.gz"],
    )

    caminhos = [
        caminho_treino_imagens,
        caminho_treino_rotulos,
        caminho_teste_imagens,
        caminho_teste_rotulos,
    ]
    if any(caminho is None for caminho in caminhos):
        raise FileNotFoundError(
            "Arquivos do MNIST nao encontrados. Veja o README em data/mnist."
        )

    # // monta dois conjuntos menores para deixar o treino viavel em Python puro.
    treino, contagem_treino = carregar_amostras_filtradas(
        caminho_treino_imagens,
        caminho_treino_rotulos,
        classes_desejadas,
        limite_treino_por_classe,
    )
    teste, contagem_teste = carregar_amostras_filtradas(
        caminho_teste_imagens,
        caminho_teste_rotulos,
        classes_desejadas,
        limite_teste_por_classe,
    )

    return treino, teste, contagem_treino, contagem_teste
