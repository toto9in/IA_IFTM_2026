from PIL import Image, ImageOps


def inverter_se_necessario(imagem):
    # // se a imagem vier com fundo claro, inverte para o padrao do MNIST.
    pixels = list(imagem.getdata())
    media = sum(pixels) / len(pixels)
    if media > 127:
        return ImageOps.invert(imagem)
    return imagem


def recortar_digito(imagem):
    # // usa um limiar simples para descobrir onde realmente existe tinta do digito.
    mascara = imagem.point(lambda pixel: 255 if pixel > 30 else 0)
    caixa = mascara.getbbox()

    # // se nao encontrar nada, devolve a propria imagem para evitar erro.
    if caixa is None:
        return imagem

    return imagem.crop(caixa)


def redimensionar_com_margem(imagem, tamanho_final):
    # // o MNIST deixa o digito menor que o quadro total, com bordas pretas ao redor.
    tamanho_digito = tamanho_final - 8

    largura, altura = imagem.size
    if largura == 0 or altura == 0:
        return Image.new("L", (tamanho_final, tamanho_final), 0)

    if largura > altura:
        nova_largura = tamanho_digito
        nova_altura = max(1, round((altura / largura) * tamanho_digito))
    else:
        nova_altura = tamanho_digito
        nova_largura = max(1, round((largura / altura) * tamanho_digito))

    imagem = imagem.resize((nova_largura, nova_altura), Image.Resampling.LANCZOS)

    tela = Image.new("L", (tamanho_final, tamanho_final), 0)
    esquerda = (tamanho_final - nova_largura) // 2
    topo = (tamanho_final - nova_altura) // 2
    tela.paste(imagem, (esquerda, topo))
    return tela


def imagem_para_matriz(imagem, tamanho):
    # // transforma a imagem em uma matriz de floats entre 0 e 1.
    matriz = []
    pixels = list(imagem.getdata())
    indice = 0
    for _linha in range(tamanho):
        linha = []
        for _coluna in range(tamanho):
            linha.append(pixels[indice] / 255.0)
            indice += 1
        matriz.append(linha)
    return matriz


def carregar_imagem_para_rede(caminho, tamanho):
    # // abre a imagem e converte para tons de cinza, igual ao formato do MNIST.
    imagem = Image.open(caminho).convert("L")

    # // aumenta o contraste para destacar melhor o traco do numero.
    imagem = ImageOps.autocontrast(imagem)

    # // deixa o digito claro em fundo escuro, como no dataset de treino.
    imagem = inverter_se_necessario(imagem)

    # // remove bordas vazias para a rede olhar mais para o numero do que para o fundo.
    imagem = recortar_digito(imagem)

    # // redimensiona o digito preservando proporcao e centraliza dentro do quadro 28x28.
    imagem = redimensionar_com_margem(imagem, tamanho)

    return imagem_para_matriz(imagem, tamanho)
