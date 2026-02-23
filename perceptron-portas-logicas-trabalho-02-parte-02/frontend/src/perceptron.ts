export type GateName = 'AND' | 'OR' | 'NAND' | 'NOR' | 'XOR'
export type Bit = 0 | 1

interface GateDef {
  X: [Bit, Bit][]
  y: Bit[]
  sep: boolean
}

export interface EpochSnapshot {
  epoch: number
  w1: number
  w2: number
  bias: number
  error: number
}

export interface TestResult {
  A: Bit
  B: Bit
  expected: Bit
  predicted: Bit
}

export interface TrainResult {
  history: EpochSnapshot[]
  testResults: TestResult[]
  converged: boolean
  finalW1: number
  finalW2: number
  finalBias: number
  finalError: number
}

export const GATES: Record<GateName, GateDef> = {
  AND:  { X: [[0,0],[0,1],[1,0],[1,1]], y: [0,0,0,1], sep: true  },
  OR:   { X: [[0,0],[0,1],[1,0],[1,1]], y: [0,1,1,1], sep: true  },
  NAND: { X: [[0,0],[0,1],[1,0],[1,1]], y: [1,1,1,0], sep: true  },
  NOR:  { X: [[0,0],[0,1],[1,0],[1,1]], y: [1,0,0,0], sep: true  },
  XOR:  { X: [[0,0],[0,1],[1,0],[1,1]], y: [0,1,1,0], sep: false },
}

function dotProduct(weights: number[], inputs: Bit[]): number {
  return weights.reduce((sum, w, i) => sum + w * inputs[i], 0)
}

function step(value: number): Bit {
  return value >= 0 ? 1 : 0
}

function round4(v: number): number {
  return Math.round(v * 10000) / 10000
}

export function trainPerceptron(
  gateName: GateName,
  epochs = 50,
  lr = 0.1,
): TrainResult {
  const gate = GATES[gateName]
  const weights: number[] = [0.0, 0.0]
  let bias = 0.0

  const predict = (inputs: Bit[]): Bit => step(dotProduct(weights, inputs) + bias)

  const history: EpochSnapshot[] = []

  for (let epoch = 0; epoch < epochs; epoch++) {
    let totalError = 0
    for (let i = 0; i < gate.X.length; i++) {
      const inputs = gate.X[i]
      const target = gate.y[i]
      const output = predict(inputs)
      const error  = target - output
      totalError  += Math.abs(error)
      weights[0]  += lr * error * inputs[0]
      weights[1]  += lr * error * inputs[1]
      bias        += lr * error
    }
    history.push({
      epoch: epoch + 1,
      w1:    round4(weights[0]),
      w2:    round4(weights[1]),
      bias:  round4(bias),
      error: totalError,
    })
  }

  const testResults: TestResult[] = gate.X.map((inputs, i) => ({
    A:         inputs[0],
    B:         inputs[1],
    expected:  gate.y[i],
    predicted: predict(inputs),
  }))

  const last = history[history.length - 1]

  return {
    history,
    testResults,
    converged:  last.error === 0,
    finalW1:    last.w1,
    finalW2:    last.w2,
    finalBias:  last.bias,
    finalError: last.error,
  }
}
