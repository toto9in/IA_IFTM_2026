import json
import os
import random


def caminho_do_projeto(*partes):
    base = os.path.dirname(os.path.abspath(__file__))
    return os.path.join(base, *partes)


def embaralhar_lista(lista):
    random.shuffle(lista)


def salvar_json(caminho, dados):
    with open(caminho, "w", encoding="utf-8") as arquivo:
        json.dump(dados, arquivo)


def carregar_json(caminho):
    with open(caminho, "r", encoding="utf-8") as arquivo:
        return json.load(arquivo)
