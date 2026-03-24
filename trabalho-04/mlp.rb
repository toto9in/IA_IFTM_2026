# frozen_string_literal: true

# MLP - Multilayer Perceptron
# Arquitetura: 3 entradas -> 2 neurônios ocultos -> 3 saídas
# Função de ativação: tanh
# Taxa de aprendizagem: 0.01

class MLP
  # Taxa de aprendizagem (eta): controla o tamanho do passo na atualização dos pesos.
  # Valores muito altos fazem a rede "pular" o mínimo do erro; muito baixos tornam o aprendizado lento.
  LEARNING_RATE = 0.01

  # Tamanhos de cada camada da rede.
  # A rede recebe 3 valores, processa por 2 neurônios ocultos e produz 3 saídas.
  N_INPUTS  = 3
  N_HIDDEN  = 2
  N_OUTPUTS = 3

  # Inicializa a rede criando as matrizes de pesos e os vetores de bias separados.
  # Os pesos começam aleatórios porque se todos começassem iguais (ex: zero),
  # todos os neurônios aprenderiam a mesma coisa e a rede nunca se especializaria.
  # O bias também começa aleatório pelo mesmo motivo.
  def initialize
    # @w_hidden: matriz [2 x 3] → 2 neurônios ocultos, cada um com 3 pesos (um por entrada)
    # Cada linha é um neurônio; cada coluna é a conexão com uma entrada específica.
    @w_hidden = random_weights(rows: N_HIDDEN, cols: N_INPUTS)

    # @bias_hidden: vetor [2] → um valor de bias por neurônio oculto, inicializado aleatoriamente.
    # O bias permite que cada neurônio desloque sua função de ativação independentemente das entradas.
    @bias_hidden = random_vector(N_HIDDEN)

    # @w_output: matriz [3 x 2] → 3 neurônios de saída, cada um com 2 pesos (um por neurônio oculto)
    @w_output = random_weights(rows: N_OUTPUTS, cols: N_HIDDEN)

    # @bias_output: vetor [3] → um valor de bias por neurônio de saída, inicializado aleatoriamente.
    @bias_output = random_vector(N_OUTPUTS)
  end

  # Treina a rede repetindo o ciclo forward → calcula erro → backprop por N épocas.
  # Uma época é uma passagem completa por todos os padrões de treinamento.
  def train(patterns:, targets:, epochs: 1000, verbose: false)
    epochs.times do |epoch|
      total_error = 0.0

      # Para cada padrão de entrada e sua respectiva saída esperada (target):
      patterns.each_with_index do |pattern, idx|
        target = targets[idx]

        # 1) Forward pass: a rede "chuta" uma resposta com os pesos atuais
        hidden_out, output_out = forward(pattern)

        # 2) Calcula o erro quadrático de cada saída: (esperado - obtido)²
        #    Dividimos pelo número de saídas para ter o erro médio deste padrão.
        #    O quadrado serve para penalizar erros grandes mais do que erros pequenos,
        #    e para que erros positivos e negativos não se cancelem.
        errors = (0...target.size).map { |i| (target[i] - output_out[i])**2 }
        total_error += errors.sum / errors.size

        # 3) Backpropagation: usa o erro para ajustar os pesos na direção certa
        backprop(pattern, target, hidden_out, output_out)
      end

      avg_error = total_error / patterns.size

      puts "Época #{epoch + 1}: Erro médio = #{avg_error.round(6)}" if verbose && ((epoch + 1) % 100).zero?
    end
  end

  def predict(pattern)
    _, output = forward(pattern)
    output
  end

  # Avalia a rede em todos os padrões e mostra se ela acertou cada um.
  # A classificação é feita pelo índice do maior valor na saída (winner-takes-all).
  def evaluate(patterns:, targets:)
    puts "\n=== Avaliação da rede ==="
    patterns.each_with_index do |pattern, idx|
      output = predict(pattern)
      target = targets[idx]
      puts "Padrão #{idx + 1}:"
      puts "  Entrada : #{pattern}"
      puts "  Esperado: #{target}"
      puts "  Obtido  : #{output.map { |v| v.round(4) }}"
    end
  end

  private

  # Cria uma matriz rows × cols preenchida com valores aleatórios em [-0.5, 0.5).
  # rand gera [0, 1), então rand - 0.5 centraliza em zero.
  # Pesos pequenos evitam que a tanh sature logo no início, o que travaria o aprendizado.
  def random_weights(rows:, cols:)
    Array.new(rows) { Array.new(cols) { rand - 0.5 } }
  end

  # Cria um vetor de tamanho n com valores aleatórios em [-0.5, 0.5).
  # Usado para inicializar os bias de cada camada.
  def random_vector(n)
    Array.new(n) { rand - 0.5 }
  end

  # Função de ativação: comprime qualquer valor real para o intervalo (-1, +1).
  # Isso é importante porque mantém os sinais controlados conforme passam pelas camadas
  # e introduz a não-linearidade que permite à rede aprender padrões complexos.
  def tanh(x)
    Math.tanh(x)
  end

  # Derivada da tanh em função da saída já calculada: tanh'(x) = 1 - tanh(x)²
  # Usada no backprop para saber "o quanto" aquele neurônio consegue aprender agora.
  # Quando a saída está perto de +1 ou -1 (saturada), a derivada fica próxima de 0
  # e o neurônio praticamente para de aprender — problema chamado de "vanishing gradient".
  def tanh_derivative(tanh_output)
    1.0 - tanh_output**2
  end

  # Forward pass: propaga o sinal da entrada até a saída, camada por camada.
  # Cada neurônio calcula uma soma ponderada das suas entradas mais seu bias, e passa pela tanh.
  def forward(pattern)
    # --- Camada oculta ---
    # Para cada neurônio oculto i, calcula o net (soma ponderada das entradas + bias próprio):
    # net[i] = w[i][0]*x[0] + w[i][1]*x[1] + w[i][2]*x[2] + bias_hidden[i]
    hidden_net = (0...N_HIDDEN).map { |i| dot(@w_hidden[i], pattern) + @bias_hidden[i] }

    # Aplica a tanh em cada net para obter a saída ativada de cada neurônio oculto.
    hidden_out = hidden_net.map { |net| tanh(net) }

    # --- Camada de saída ---
    # Repete o mesmo processo usando as saídas da camada oculta como entrada:
    # net[i] = w[i][0]*h[0] + w[i][1]*h[1] + bias_output[i]
    output_net = (0...N_OUTPUTS).map { |i| dot(@w_output[i], hidden_out) + @bias_output[i] }
    output_out = output_net.map { |net| tanh(net) }

    [hidden_out, output_out]
  end

  # Backpropagation: calcula o quanto cada peso contribuiu para o erro
  # e os ajusta para reduzir esse erro na próxima passagem.
  # O processo vai da camada de saída em direção à entrada (daí "back" propagation).
  def backprop(pattern, target, hidden_out, output_out)
    # --- Passo 1: delta da camada de saída ---
    # O delta representa o "sinal de erro" de cada neurônio de saída.
    # delta = (esperado - obtido) × tanh'(saída)
    #
    # (esperado - obtido): o quanto erramos e em qual direção
    output_deltas = (0...target.size).map do |i|
      (target[i] - output_out[i]) * tanh_derivative(output_out[i])
    end

    # --- Passo 2: delta da camada oculta --
    # error_sum = Σ (delta_saída[k] × w_output[k][j]) — erro ponderado pelos pesos
    # delta_oculto[j] = error_sum × tanh'(saída_oculta[j])
    #
    hidden_deltas = hidden_out.each_with_index.map do |h_out, j|
      error_sum = output_deltas.each_with_index.sum { |delta, k| delta * @w_output[k][j] }
      error_sum * tanh_derivative(h_out)
    end

    # --- Passo 3: atualiza os pesos e o bias da camada de saída ---
    @w_output.each_with_index do |weights, i|
      weights.each_with_index do |_, j|
        @w_output[i][j] += LEARNING_RATE * output_deltas[i] * hidden_out[j]
      end
      @bias_output[i] += LEARNING_RATE * output_deltas[i]
    end

    # --- Passo 4: atualiza os pesos e o bias da camada oculta ---
    # Mesma regra delta, usando os deltas dos neurônios ocultos e as entradas originais.
    # O bias da camada oculta também é atualizado pelo seu próprio delta.
    @w_hidden.each_with_index do |weights, i|
      weights.each_with_index do |_, j|
        @w_hidden[i][j] += LEARNING_RATE * hidden_deltas[i] * pattern[j]
      end
      @bias_hidden[i] += LEARNING_RATE * hidden_deltas[i]
    end
  end

  # Produto escalar entre dois vetores: Σ a[i] * b[i]
  # É a operação que cada neurônio executa para combinar suas entradas com seus pesos.
  def dot(a, b)
    (0...a.size).sum { |i| a[i] * b[i] }
  end
end
