"use strict";

const CIDADES = [
  "Uberlândia",
  "Uberaba",
  "Araguari",
  "Ituiutaba",
  "Patos de Minas",
  "Frutal",
  "Araxá",
  "Monte Carmelo",
  "Tupaciguara",
  "Campina Verde",
];
const NUM_CIDADES = CIDADES.length;
const ORIGEM = 1; // índice de Uberaba

// Coordenadas geográficas aproximadas (lat, lon) só para desenhar o mapa.
// As distâncias reais vêm da matriz abaixo, não destas coordenadas. (ajuda IA)
const COORDENADAS = [
  [-18.91, -48.27], // Uberlândia
  [-19.75, -47.93], // Uberaba
  [-18.65, -48.19], // Araguari
  [-18.97, -49.46], // Ituiutaba
  [-18.58, -46.51], // Patos de Minas
  [-20.02, -48.94], // Frutal
  [-19.59, -46.94], // Araxá
  [-18.72, -47.5], // Monte Carmelo
  [-18.59, -48.7], // Tupaciguara
  [-19.53, -49.49], // Campina Verde
];

// Matriz simétrica; SEM = não existe estrada direta (— na tabela do enunciado).
const SEM = null;
const DISTANCIA_DIRETA = [
  //           Ubl   Uba   Arg   Itu   Pat   Fru   Arx   MtC   Tup   CmV
  /*Ubl*/ [0, 106, 30, 138, 190, SEM, 175, 89, 56, SEM],
  /*Uba*/ [106, 0, SEM, SEM, 265, 105, 110, SEM, SEM, 160],
  /*Arg*/ [30, SEM, 0, 117, 221, SEM, 205, 66, 47, SEM],
  /*Itu*/ [138, SEM, 117, 0, SEM, 265, SEM, 163, SEM, 186],
  /*Pat*/ [190, 265, 221, SEM, 0, SEM, 137, 114, SEM, SEM],
  /*Fru*/ [SEM, 105, SEM, 265, SEM, 0, 185, SEM, SEM, 75],
  /*Arx*/ [175, 110, 205, SEM, 137, 185, 0, 145, SEM, SEM],
  /*MtC*/ [89, SEM, 66, 163, 114, SEM, 145, 0, 111, SEM],
  /*Tup*/ [56, SEM, 47, SEM, SEM, SEM, SEM, 111, 0, SEM],
  /*CmV*/ [SEM, 160, SEM, 186, SEM, 75, SEM, SEM, SEM, 0],
];

const INFINITO = Infinity;
let menorDistancia; // menorDistancia[i][j] = menor distância de i até j
let proximaNoCaminho; // proximaNoCaminho[i][j] = próxima cidade indo de i até j

function floydWarshall() {
  menorDistancia = DISTANCIA_DIRETA.map((linha) =>
    linha.map((valor) => (valor === null ? INFINITO : valor)),
  );
  proximaNoCaminho = DISTANCIA_DIRETA.map((linha, origem) =>
    linha.map((valor, destino) =>
      valor === null || origem === destino
        ? origem === destino
          ? origem
          : null
        : destino,
    ),
  );
  for (let cidade = 0; cidade < NUM_CIDADES; cidade++)
    menorDistancia[cidade][cidade] = 0;

  for (let intermediaria = 0; intermediaria < NUM_CIDADES; intermediaria++)
    for (let origem = 0; origem < NUM_CIDADES; origem++)
      for (let destino = 0; destino < NUM_CIDADES; destino++) {
        const distanciaViaIntermediaria =
          menorDistancia[origem][intermediaria] +
          menorDistancia[intermediaria][destino];
        if (distanciaViaIntermediaria < menorDistancia[origem][destino]) {
          menorDistancia[origem][destino] = distanciaViaIntermediaria;
          proximaNoCaminho[origem][destino] =
            proximaNoCaminho[origem][intermediaria];
        }
      }
}

function caminhoIntermediario(origem, destino) {
  if (proximaNoCaminho[origem][destino] === null) return null;
  const intermediarias = [];
  let atual = origem;
  while (atual !== destino) {
    atual = proximaNoCaminho[atual][destino];
    if (atual !== destino) intermediarias.push(atual);
  }
  return intermediarias;
}

const inteiroAleatorio = (limite) => Math.floor(Math.random() * limite);

function embaralhar(lista) {
  const copia = lista.slice();
  for (let posicao = copia.length - 1; posicao > 0; posicao--) {
    const sorteada = inteiroAleatorio(posicao + 1);
    [copia[posicao], copia[sorteada]] = [copia[sorteada], copia[posicao]];
  }
  return copia;
}

function rotaCompleta(cromossomo, destino) {
  return [ORIGEM, ...cromossomo, destino];
}

function distanciaRota(cromossomo, destino) {
  const rota = rotaCompleta(cromossomo, destino);
  let total = 0;
  for (let passo = 0; passo < rota.length - 1; passo++) {
    const trecho = menorDistancia[rota[passo]][rota[passo + 1]];
    if (trecho === INFINITO) return INFINITO;
    total += trecho;
  }
  return total;
}

function crossoverOX(pai1, pai2) {
  const tamanho = pai1.length;
  if (tamanho < 2) return pai1.slice();
  const corte1 = inteiroAleatorio(tamanho),
    corte2 = inteiroAleatorio(tamanho);
  const inicio = Math.min(corte1, corte2),
    fim = Math.max(corte1, corte2);

  const filho = new Array(tamanho).fill(null);
  // 1) copia o segmento [inicio..fim] do pai1
  for (let posicao = inicio; posicao <= fim; posicao++)
    filho[posicao] = pai1[posicao];
  const cidadesNoSegmento = new Set(filho.slice(inicio, fim + 1));

  // 2) preenche o resto na ordem do pai2, pulando cidades já presentes
  let posicaoFilho = (fim + 1) % tamanho;
  for (let deslocamento = 0; deslocamento < tamanho; deslocamento++) {
    const cidade = pai2[(fim + 1 + deslocamento) % tamanho];
    if (!cidadesNoSegmento.has(cidade)) {
      filho[posicaoFilho] = cidade;
      posicaoFilho = (posicaoFilho + 1) % tamanho;
    }
  }
  return filho;
}

function mutarTroca(cromossomo) {
  const tamanho = cromossomo.length;
  if (tamanho < 2) return;
  const posicaoA = inteiroAleatorio(tamanho);
  let posicaoB = inteiroAleatorio(tamanho);
  while (posicaoB === posicaoA) posicaoB = inteiroAleatorio(tamanho);
  [cromossomo[posicaoA], cromossomo[posicaoB]] = [
    cromossomo[posicaoB],
    cromossomo[posicaoA],
  ];
}

function probabilidadesRanking(distancias, metodo, pressao) {
  const n = distancias.length;
  const ordenados = distancias
    .map((_, indice) => indice)
    .sort((a, b) => distancias[a] - distancias[b]);

  const probabilidade = new Array(n);
  if (metodo === "exponencial") {
    const c = 1 - 0.9 * pressao;
    let soma = 0;
    ordenados.forEach((indice, posicao) => {
      const peso = Math.pow(c, posicao);
      probabilidade[indice] = peso;
      soma += peso;
    });
    for (let i = 0; i < n; i++) probabilidade[i] /= soma;
  } else {
    const sp = 1 + pressao;
    ordenados.forEach((indice, posicao) => {
      const fracao = n > 1 ? posicao / (n - 1) : 0;
      probabilidade[indice] = (sp - (2 * sp - 2) * fracao) / n;
    });
  }
  return probabilidade;
}

function selecaoRanking(populacao, probabilidades) {
  let sorteio = Math.random();
  for (let i = 0; i < populacao.length; i++) {
    sorteio -= probabilidades[i];
    if (sorteio <= 0) return populacao[i];
  }
  return populacao[populacao.length - 1];
}

function cidadesLivres(destino) {
  const livres = [];
  for (let cidade = 0; cidade < NUM_CIDADES; cidade++) {
    if (cidade === ORIGEM) continue;
    if (destino !== ORIGEM && cidade === destino) continue;
    livres.push(cidade);
  }
  return livres;
}

let loopAnimacao = null;

function rodarAG(config, aoAvancarGeracao, aoFinalizar) {
  const livres = cidadesLivres(config.destino);

  let populacao = [];
  for (let individuo = 0; individuo < config.populacao; individuo++) {
    populacao.push(embaralhar(livres));
  }

  let melhorRota = null,
    melhorDistancia = INFINITO,
    geracaoDoMelhor = 0;
  const historicoDistancias = [];
  let geracao = 0;

  function passo() {
    // avalia a distância de cada indivíduo
    const distancias = populacao.map((individuo) =>
      distanciaRota(individuo, config.destino),
    );

    // melhor indivíduo desta geração
    let indiceMelhor = 0;
    for (let i = 1; i < populacao.length; i++) {
      if (distancias[i] < distancias[indiceMelhor]) indiceMelhor = i;
    }
    if (distancias[indiceMelhor] < melhorDistancia) {
      melhorDistancia = distancias[indiceMelhor];
      melhorRota = populacao[indiceMelhor].slice();
      geracaoDoMelhor = geracao;
    }
    historicoDistancias.push(distancias[indiceMelhor]);

    aoAvancarGeracao({
      geracao,
      melhorRota,
      melhorDistancia,
      geracaoDoMelhor,
      historicoDistancias,
      destino: config.destino,
    });

    if (geracao >= config.geracoes - 1) {
      aoFinalizar({
        melhorRota,
        melhorDistancia,
        geracaoDoMelhor,
        destino: config.destino,
      });
      return false;
    }

    const indicesOrdenados = populacao
      .map((_, indice) => indice)
      .sort((a, b) => distancias[a] - distancias[b]);

    const probabilidades = probabilidadesRanking(
      distancias,
      config.metodo,
      config.pressao,
    );

    const proximaPopulacao = [];
    for (let e = 0; e < config.elitismo && e < indicesOrdenados.length; e++) {
      proximaPopulacao.push(populacao[indicesOrdenados[e]].slice());
    }

    while (proximaPopulacao.length < config.populacao) {
      const pai1 = selecaoRanking(populacao, probabilidades);
      const pai2 = selecaoRanking(populacao, probabilidades);
      let filho =
        Math.random() < config.txCruzamento
          ? crossoverOX(pai1, pai2)
          : pai1.slice();
      if (Math.random() < config.txMutacao) mutarTroca(filho);
      proximaPopulacao.push(filho);
    }

    populacao = proximaPopulacao;
    geracao++;
    return true;
  }

  // loop animado (uma geração por quadro, ou com atraso configurável)
  function avancar() {
    const continuar = passo();
    if (continuar) {
      loopAnimacao =
        config.delay > 0
          ? setTimeout(avancar, config.delay)
          : requestAnimationFrame(avancar);
    } else {
      loopAnimacao = null;
    }
  }
  avancar();
}

function pararAG() {
  if (loopAnimacao !== null) {
    clearTimeout(loopAnimacao);
    cancelAnimationFrame(loopAnimacao);
    loopAnimacao = null;
  }
}

// SECAO DE RENDERE
const elemento = (id) => document.getElementById(id);

// mapa Leaflet criado uma vez; camadas redesenhadas a cada geração
let mapaLeaflet = null;
let camadaCidades = null;
let camadaRota = null;
let camadaEstrada = null;
let ultimoDestinoDesenhado = null;

function inicializarMapa() {
  mapaLeaflet = L.map("map");
  L.tileLayer("https://{s}.basemaps.cartocdn.com/dark_all/{z}/{x}/{y}{r}.png", {
    attribution: "&copy; OpenStreetMap &copy; CARTO",
    maxZoom: 19,
  }).addTo(mapaLeaflet);
  mapaLeaflet.fitBounds(COORDENADAS, { padding: [40, 40] });
  camadaCidades = L.layerGroup().addTo(mapaLeaflet);
  camadaRota = L.layerGroup().addTo(mapaLeaflet);
  camadaEstrada = L.layerGroup().addTo(mapaLeaflet);
}

// rota seguindo estradas reais (OSRM) — só para exibição; distâncias vêm da tabela
async function desenharRotaEstrada(rota) {
  camadaEstrada.clearLayers();
  // OSRM espera pares lon,lat separados por ponto e vírgula
  const pontos = rota
    .map((cidade) => `${COORDENADAS[cidade][1]},${COORDENADAS[cidade][0]}`)
    .join(";");
  const url = `https://router.project-osrm.org/route/v1/driving/${pontos}?overview=full&geometries=geojson`;
  try {
    const resposta = await fetch(url);
    const dados = await resposta.json();
    if (dados.code !== "Ok" || !dados.routes.length)
      throw new Error(dados.code);
    // GeoJSON vem em [lon, lat]; Leaflet usa [lat, lon]
    const linha = dados.routes[0].geometry.coordinates.map((c) => [c[1], c[0]]);
    L.polyline(linha, { color: "#33d9a6", weight: 4, opacity: 0.9 }).addTo(
      camadaEstrada,
    );
  } catch (erro) {
    console.warn("OSRM indisponível — mantendo linha reta:", erro);
  }
}

// marcadores das cidades (origem, destino e demais em cores distintas)
function desenharCidades(destino) {
  camadaCidades.clearLayers();
  COORDENADAS.forEach((posicao, cidade) => {
    let cor = "#7fb3ff",
      raio = 6;
    if (cidade === ORIGEM) {
      cor = "#33d9a6";
      raio = 8;
    } else if (destino !== undefined && cidade === destino) {
      cor = "#ffb454";
      raio = 8;
    }
    L.circleMarker(posicao, {
      radius: raio,
      color: "#0f1420",
      weight: 2,
      fillColor: cor,
      fillOpacity: 1,
    })
      .bindTooltip(CIDADES[cidade], {
        permanent: true,
        direction: "bottom",
        className: "rotulo-cidade",
        offset: [0, 6],
      })
      .addTo(camadaCidades);
  });
}

function desenharMapa(cromossomo, destino) {
  // cidades só são refeitas quando o destino muda (evita churn na animação)
  if (destino !== ultimoDestinoDesenhado) {
    desenharCidades(destino);
    ultimoDestinoDesenhado = destino;
  }
  camadaRota.clearLayers();
  if (!cromossomo) return;

  const rota = rotaCompleta(cromossomo, destino);

  // sequência real de cidades, incluindo intermediárias do Floyd-Warshall
  const sequenciaReal = [rota[0]];
  for (let passo = 0; passo < rota.length - 1; passo++) {
    const intermediarias =
      caminhoIntermediario(rota[passo], rota[passo + 1]) || [];
    sequenciaReal.push(...intermediarias, rota[passo + 1]);
  }
  // linha do caminho real (tracejada) — mostra por onde o caminhão passa de fato
  L.polyline(
    sequenciaReal.map((cidade) => COORDENADAS[cidade]),
    { color: "#93a2bc", weight: 2, dashArray: "5 6", opacity: 0.8 },
  ).addTo(camadaRota);

  // linha da rota lógica (direta, em destaque)
  L.polyline(
    rota.map((cidade) => COORDENADAS[cidade]),
    { color: "#5e9bff", weight: 3, opacity: 0.9 },
  ).addTo(camadaRota);

  // números da ordem de visita
  rota.forEach((cidade, ordem) => {
    if (ordem === rota.length - 1 && destino === ORIGEM) return; // não repete rótulo no fecho do ciclo
    L.marker(COORDENADAS[cidade], {
      icon: L.divIcon({
        className: "",
        html: `<div class="ordem-visita">${ordem}</div>`,
        iconSize: [20, 20],
      }),
      interactive: false,
    }).addTo(camadaRota);
  });
}

function desenharGrafico(historicoDistancias) {
  const svg = elemento("chart");
  const largura = 800,
    altura = 240,
    margem = 40;
  if (!historicoDistancias.length) {
    svg.innerHTML = "";
    return;
  }

  const finitos = historicoDistancias.filter((v) => v !== INFINITO);
  const maximo = finitos.length ? Math.max(...finitos) : 1;
  const minimo = finitos.length ? Math.min(...finitos) : 0;
  const amplitude = maximo - minimo || 1;
  const total = historicoDistancias.length;

  let linhaSvg = "";
  historicoDistancias.forEach((valor, indice) => {
    if (valor === INFINITO) return;
    const x =
      margem +
      (total === 1 ? 0 : (indice / (total - 1)) * (largura - 2 * margem));
    const y =
      altura - margem - ((valor - minimo) / amplitude) * (altura - 2 * margem);
    linhaSvg +=
      (linhaSvg === "" ? "M" : "L") + x.toFixed(1) + " " + y.toFixed(1) + " ";
  });

  let svgHtml = "";
  // eixos
  svgHtml += `<line x1="${margem}" y1="${altura - margem}" x2="${largura - margem}" y2="${altura - margem}" stroke="#2c3a52"/>`;
  svgHtml += `<line x1="${margem}" y1="${margem}" x2="${margem}" y2="${altura - margem}" stroke="#2c3a52"/>`;
  // rótulos
  svgHtml += `<text x="${margem - 6}" y="${margem + 4}" text-anchor="end" font-size="11" fill="#93a2bc">${Math.round(maximo)}</text>`;
  svgHtml += `<text x="${margem - 6}" y="${altura - margem}" text-anchor="end" font-size="11" fill="#93a2bc">${Math.round(minimo)}</text>`;
  svgHtml += `<text x="${largura - margem}" y="${altura - margem + 18}" text-anchor="end" font-size="11" fill="#93a2bc">geração ${total - 1}</text>`;
  svgHtml += `<text x="${margem}" y="${altura - margem + 18}" text-anchor="start" font-size="11" fill="#93a2bc">0</text>`;
  svgHtml += `<path d="${linhaSvg}" fill="none" stroke="#33d9a6" stroke-width="2"/>`;
  svg.innerHTML = svgHtml;
}

function textoRota(cromossomo, destino, distancia) {
  if (distancia === INFINITO) {
    return `<span class="warn">Rota inválida (trecho inalcançável).</span>`;
  }
  const nomes = rotaCompleta(cromossomo, destino).map(
    (cidade) => CIDADES[cidade],
  );
  return (
    nomes.join('<span class="arrow">→</span>') +
    `<br><strong>Distância total: ${Math.round(distancia)} km</strong>`
  );
}

// ---------- popular dropdown de destino ----------
(function inicializarDestino() {
  const seletor = elemento("destino");
  CIDADES.forEach((nome, indice) => {
    const opcao = document.createElement("option");
    opcao.value = indice;
    opcao.textContent =
      nome + (indice === ORIGEM ? " (volta à origem / ciclo)" : "");
    seletor.appendChild(opcao);
  });
  seletor.value = ORIGEM; // default: rota fechada
})();

// ---------- sliders com valor ao vivo ----------
const sliders = [
  ["pop", "pop-v", (valor) => valor],
  ["ger", "ger-v", (valor) => valor],
  ["txc", "txc-v", (valor) => Number(valor).toFixed(2)],
  ["txm", "txm-v", (valor) => Number(valor).toFixed(2)],
  ["elit", "elit-v", (valor) => valor],
  ["pressao", "pressao-v", (valor) => Number(valor).toFixed(2)],
  ["vel", "vel-v", (valor) => (Number(valor) === 0 ? "rápida" : valor + "ms")],
];
sliders.forEach(([idSlider, idValor, formatar]) => {
  const slider = elemento(idSlider);
  slider.addEventListener("input", () => {
    elemento(idValor).textContent = formatar(slider.value);
  });
});

// ---------- destino muda → redesenha mapa base ----------
elemento("destino").addEventListener("change", () => {
  camadaEstrada.clearLayers();
  desenharMapa(null, Number(elemento("destino").value));
});

// ---------- botões ----------
elemento("run").addEventListener("click", () => {
  pararAG();
  const config = {
    destino: Number(elemento("destino").value),
    populacao: Number(elemento("pop").value),
    geracoes: Number(elemento("ger").value),
    txCruzamento: Number(elemento("txc").value),
    txMutacao: Number(elemento("txm").value),
    elitismo: Number(elemento("elit").value),
    metodo: document.querySelector('input[name="metodo"]:checked').value,
    pressao: Number(elemento("pressao").value),
    delay: Number(elemento("vel").value),
  };
  elemento("run").disabled = true;
  elemento("stop").disabled = false;
  camadaEstrada.clearLayers(); // limpa rota por estradas da execução anterior

  rodarAG(
    config,
    // a cada geração
    (estado) => {
      elemento("s-dist").textContent =
        estado.melhorDistancia === INFINITO
          ? "—"
          : Math.round(estado.melhorDistancia);
      elemento("s-ger").textContent = estado.geracao;
      elemento("s-melhorger").textContent = estado.geracaoDoMelhor;
      desenharMapa(estado.melhorRota, estado.destino);
      desenharGrafico(estado.historicoDistancias);
      elemento("route-text").innerHTML = textoRota(
        estado.melhorRota,
        estado.destino,
        estado.melhorDistancia,
      );
    },
    // no fim
    (final) => {
      elemento("run").disabled = false;
      elemento("stop").disabled = true;
      // desenha a melhor rota seguindo estradas reais (uma única chamada OSRM)
      if (final.melhorDistancia !== INFINITO) {
        desenharRotaEstrada(rotaCompleta(final.melhorRota, final.destino));
      }
    },
  );
});

elemento("stop").addEventListener("click", () => {
  pararAG();
  elemento("run").disabled = false;
  elemento("stop").disabled = true;
});

// ---------- inicialização ----------
floydWarshall();
inicializarMapa();
desenharMapa(null, ORIGEM);
desenharGrafico([]);
