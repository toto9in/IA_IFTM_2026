# frozen_string_literal: true

require_relative 'mlp'

PATTERNS = [
  [1,  0.5, -1],
  [0,  0.5,  1],
  [1, -0.5, -1]
].freeze

TARGETS = [
  [1, -1, -1],
  [-1,  1, -1],
  [-1, -1,  1]
].freeze

puts 'Inicializando rede MLP...'
puts 'Arquitetura: 3 entradas -> 2 neurônios ocultos -> 3 saídas'
puts "Taxa de aprendizagem: #{MLP::LEARNING_RATE}"
puts "Função de ativação: tanh\n\n"

mlp = MLP.new

puts 'Treinando por 10.000 épocas...'
mlp.train(patterns: PATTERNS, targets: TARGETS, epochs: 100_000, verbose: true)

mlp.evaluate(patterns: PATTERNS, targets: TARGETS)
