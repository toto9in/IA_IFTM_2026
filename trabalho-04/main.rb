# frozen_string_literal: true

require_relative 'mlp'

puts 'Inicializando rede MLP...'
puts 'Arquitetura: 3 entradas -> 2 neurônios ocultos -> 3 saídas'
puts "Taxa de aprendizagem: #{MLP::LEARNING_RATE}"
puts "Função de ativação: tanh\n\n"

mlp = MLP.new

puts 'Treinando por 10.000 épocas...'
mlp.train(epochs: 100_000, verbose: true)

mlp.evaluate
