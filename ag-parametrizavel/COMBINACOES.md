# Combinações de Parâmetros — AG Parametrizável

Função minimizada: **f(x) = −|x · sin(√|x|)|**, domínio [0, 512]

Mínimo global esperado: x ≈ 420.97, f(x) ≈ −418.98

---

## Parâmetros de entrada (ordem da solicitação)

```
1. Bits do cromossomo
2. Tamanho da população
3. % da população para cruzamento (0.0 a 1.0)
4. Probabilidade de mutação (0.0 a 1.0)
5. Máximo de gerações
6. Método de seleção: 1 = Roleta, 2 = Torneio
7. (se Torneio) Tamanho do torneio
8. Método de cruzamento: 1 = Um ponto, 2 = Dois pontos
```

---

## Configuração base (equivalente ao ag-funcao original)

```
Bits:         10
População:    100
Cruzamento:   0.20   (20% = 20 indivíduos, como o original)
Mutação:      0.01
Gerações:     50
Seleção:      1 (Roleta)
Cruzamento:   1 (Um ponto)
```

---

## Combinação 1 — Padrão recomendado

```
Bits:         10
População:    100
Cruzamento:   0.80
Mutação:      0.01
Gerações:     50
Seleção:      1 (Roleta)
Cruzamento:   1 (Um ponto)
```

Comportamento esperado: convergência gradual, boa diversidade.

---

## Combinação 2 — Torneio pequeno (pressão seletiva moderada)

```
Bits:         10
População:    100
Cruzamento:   0.80
Mutação:      0.01
Gerações:     50
Seleção:      2 (Torneio)
Tamanho torneio: 3
Cruzamento:   1 (Um ponto)
```

Comportamento esperado: convergência um pouco mais rápida que a roleta.

---

## Combinação 3 — Torneio grande (alta pressão seletiva)

```
Bits:         10
População:    100
Cruzamento:   0.80
Mutação:      0.01
Gerações:     50
Seleção:      2 (Torneio)
Tamanho torneio: 20
Cruzamento:   1 (Um ponto)
```

Comportamento esperado: convergência muito rápida, risco de mínimo local.

---

## Combinação 4 — Dois pontos de cruzamento

```
Bits:         10
População:    100
Cruzamento:   0.80
Mutação:      0.01
Gerações:     50
Seleção:      2 (Torneio)
Tamanho torneio: 5
Cruzamento:   2 (Dois pontos)
```

Comportamento esperado: mais troca de material genético por cruzamento.

---

## Combinação 5 — Alta mutação (exploração)

```
Bits:         10
População:    100
Cruzamento:   0.80
Mutação:      0.10
Gerações:     100
Seleção:      1 (Roleta)
Cruzamento:   1 (Um ponto)
```

Comportamento esperado: oscilação alta, difícil convergir — mutação destrói boas soluções.

---

## Combinação 6 — Mutação quase zero (exploração mínima)

```
Bits:         10
População:    100
Cruzamento:   0.80
Mutação:      0.001
Gerações:     100
Seleção:      1 (Roleta)
Cruzamento:   1 (Um ponto)
```

Comportamento esperado: convergência rápida mas pode prender em mínimo local.

---

## Combinação 7 — Cromossomo maior (mais precisão)

```
Bits:         16
População:    100
Cruzamento:   0.80
Mutação:      0.01
Gerações:     100
Seleção:      1 (Roleta)
Cruzamento:   2 (Dois pontos)
```

Comportamento esperado: resolução maior no espaço de busca, resultado mais próximo do mínimo global.

---

## Combinação 8 — Cromossomo menor (menos precisão)

```
Bits:         5
População:    100
Cruzamento:   0.80
Mutação:      0.01
Gerações:     50
Seleção:      1 (Roleta)
Cruzamento:   1 (Um ponto)
```

Comportamento esperado: apenas 32 valores possíveis de x, solução imprecisa.

---

## Combinação 9 — População pequena

```
Bits:         10
População:    20
Cruzamento:   0.80
Mutação:      0.05
Gerações:     100
Seleção:      2 (Torneio)
Tamanho torneio: 3
Cruzamento:   1 (Um ponto)
```

Comportamento esperado: convergência prematura comum, alta variância entre execuções.

---

## Comparação direta para apresentação

| # | Pop | Bits | Mutação | Seleção        | Cruzamento  | Resultado típico |
|---|-----|------|---------|----------------|-------------|------------------|
| 1 | 100 |  10  | 0.01    | Roleta         | 1 ponto     | Bom              |
| 2 | 100 |  10  | 0.01    | Torneio k=3    | 1 ponto     | Bom              |
| 3 | 100 |  10  | 0.01    | Torneio k=20   | 1 ponto     | Rápido/local     |
| 4 | 100 |  10  | 0.10    | Roleta         | 1 ponto     | Oscila muito     |
| 5 | 100 |  16  | 0.01    | Roleta         | 2 pontos    | Mais preciso     |
| 6 |  20 |  10  | 0.05    | Torneio k=3    | 1 ponto     | Instável         |
