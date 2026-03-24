# frozen_string_literal: true

# MLP - Multilayer Perceptron
# Arquitetura: 3 entradas -> 2 neurônios ocultos -> 3 saídas
# Função de ativação: tanh
# Taxa de aprendizagem: 0.01

class MLP
  LEARNING_RATE = 0.01

  N_INPUTS  = 3
  N_HIDDEN  = 2
  N_OUTPUTS = 3

  # Padrões de entrada (3 padrões, 3 features cada)
  PATTERNS = [
    [1,  0.5, -1],
    [0,  0.5,  1],
    [1, -0.5, -1]
  ].freeze

  # Targets (saída esperada para cada padrão)
  # Padrão 1: [1, -1, -1]
  # Padrão 2: [-1, 1, -1]
  # Padrão 3: [-1, -1, 1]
  TARGETS = [
    [1, -1, -1],
    [-1,  1, -1],
    [-1, -1,  1]
  ].freeze

  def initialize
    # rows = quantos neurônios recebem os pesos (quem calcula)
    # cols = quantas conexões cada neurônio tem (de onde vem + bias)
    @w_hidden = random_weights(rows: N_HIDDEN,  cols: N_INPUTS + 1)
    @w_output = random_weights(rows: N_OUTPUTS, cols: N_HIDDEN + 1)
  end

  # Treina a rede por um número de épocas
  def train(epochs: 1000, verbose: false)
    epochs.times do |epoch|
      total_error = 0.0

      PATTERNS.each_with_index do |pattern, idx|
        target = TARGETS[idx]

        # Forward pass
        hidden_out, output_out = forward(pattern)

        # Calcula erro quadrático médio
        errors = (0...target.size).map { |i| (target[i] - output_out[i])**2 }
        total_error += errors.sum / errors.size

        # Backward pass (backpropagation)
        backprop(pattern, target, hidden_out, output_out)
      end

      avg_error = total_error / PATTERNS.size
      puts "Época #{epoch + 1}: Erro médio = #{avg_error.round(6)}" if verbose && ((epoch + 1) % 100).zero?
    end
  end

  # Faz predição para um padrão de entrada
  def predict(pattern)
    _, output = forward(pattern)
    output
  end

  # Testa todos os padrões e exibe resultados
  def evaluate
    puts "\n=== Avaliação da rede ==="
    PATTERNS.each_with_index do |pattern, idx|
      output = predict(pattern)
      target = TARGETS[idx]
      puts "Padrão #{idx + 1}:"
      puts "  Entrada : #{pattern}"
      puts "  Esperado: #{target}"
      puts "  Obtido  : #{output.map { |v| v.round(4) }}"
      puts "  Correto?: #{classify(output) == classify(target)}"
    end
  end

  private

  # Inicializa pesos aleatórios entre -0.5 e 0.5
  def random_weights(rows:, cols:)
    Array.new(rows) { Array.new(cols) { rand - 0.5 } }
  end

  # Função de ativação tanh
  def tanh(x)
    Math.tanh(x)
  end

  # Derivada de tanh: 1 - tanh(x)^2
  def tanh_derivative(tanh_output)
    1.0 - tanh_output**2
  end

  # Forward pass: calcula saídas da camada oculta e de saída
  def forward(pattern)
    input_with_bias = pattern + [1.0] # adiciona bias

    # Camada oculta
    hidden_net = @w_hidden.map { |weights| dot(weights, input_with_bias) }
    hidden_out = hidden_net.map { |net| tanh(net) }

    hidden_with_bias = hidden_out + [1.0] # adiciona bias

    # Camada de saída
    output_net = @w_output.map { |weights| dot(weights, hidden_with_bias) }
    output_out = output_net.map { |net| tanh(net) }

    [hidden_out, output_out]
  end

  # Backpropagation: atualiza os pesos
  def backprop(pattern, target, hidden_out, output_out)
    input_with_bias = pattern + [1.0]
    hidden_with_bias = hidden_out + [1.0]

    # Deltas da camada de saída
    output_deltas = (0...target.size).map do |i|
      (target[i] - output_out[i]) * tanh_derivative(output_out[i])
    end

    # Atualiza pesos da camada de saída
    @w_output.each_with_index do |weights, i|
      weights.each_with_index do |_, j|
        @w_output[i][j] += LEARNING_RATE * output_deltas[i] * hidden_with_bias[j]
      end
    end

    # Deltas da camada oculta (backprop do erro)
    hidden_deltas = hidden_out.each_with_index.map do |h_out, j|
      error_sum = output_deltas.each_with_index.sum { |delta, k| delta * @w_output[k][j] }
      error_sum * tanh_derivative(h_out)
    end

    # Atualiza pesos da camada oculta
    @w_hidden.each_with_index do |weights, i|
      weights.each_with_index do |_, j|
        @w_hidden[i][j] += LEARNING_RATE * hidden_deltas[i] * input_with_bias[j]
      end
    end
  end

  # Produto escalar entre dois vetores
  def dot(a, b)
    (0...a.size).sum { |i| a[i] * b[i] }
  end

  # Classifica a saída pelo índice do maior valor
  def classify(output)
    output.each_with_index.max_by { |v, _| v }[1]
  end
end
