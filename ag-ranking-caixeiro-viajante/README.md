# Caixeiro Viajante por Algoritmo Genético — Seleção por Ranking

Roteamento de um caminhão que parte de **Uberaba** e visita cidades do Triângulo Mineiro,
buscando a menor distância. Resolve o Problema do Caixeiro Viajante (TSP) com Algoritmo
Genético e mostra a rota em um mapa real (Leaflet + OpenStreetMap).

Variação do AG clássico: a **seleção não usa o valor absoluto do fitness**, e sim a
**posição (rank)** do indivíduo na ordenação. Dois métodos disponíveis em tempo de execução:

- **Ranking linear** — probabilidade decresce linearmente com a posição (pressão `sp` em [1, 2]).
- **Ranking exponencial** — probabilidade ∝ `c^posição` (decaimento geométrico).

O controle **Pressão de seleção** (0 a 1) é normalizado e re-escalado por método: 0 = quase
uniforme, 1 = pressão máxima.

## Como rodar

O projeto é só HTML + JavaScript, **não precisa instalar nada nem compilar**.

### Opção 1 — abrir direto (mais simples)

Dê um duplo clique em `index.html`, ou pelo terminal:

```bash
xdg-open index.html      # Linux
```

### Opção 2 — servir localmente (recomendado)

Se o mapa ou as rotas não carregarem ao abrir o arquivo direto, sirva a pasta:

```bash
python3 -m http.server 8000
```

Depois abra no navegador: <http://localhost:8000>

## Requisitos

- Um navegador moderno (Chrome, Firefox, Edge...).
- **Conexão com a internet** — usada para:
  - carregar a biblioteca de mapa (Leaflet, via CDN);
  - baixar os tiles do mapa (OpenStreetMap / CartoDB);
  - traçar a rota por estradas reais (serviço OSRM).

Sem internet o algoritmo ainda roda e calcula a melhor rota, mas o mapa fica em branco.

## Como usar

1. Escolha a **cidade de destino** (a origem é sempre Uberaba; escolher Uberaba fecha o ciclo).
2. Ajuste os parâmetros do AG nos controles à esquerda: tamanho da população, número de
   gerações, taxa de cruzamento, taxa de mutação, elitismo, método de ranking (linear ou
   exponencial) e pressão de seleção.
3. Clique em **▶ Rodar AG**.
4. Acompanhe a melhor rota se formando no mapa e a evolução da distância no gráfico.

## Arquivos

- `index.html` — interface e estilo.
- `algoritmo-genetico.js` — dados das cidades, algoritmo genético e desenho do mapa.
