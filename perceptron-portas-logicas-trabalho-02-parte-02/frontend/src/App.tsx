import { useState, useEffect, useRef, useCallback } from 'react'
import { trainPerceptron, GATES, type GateName, type EpochSnapshot, type TestResult, type TrainResult } from './perceptron'
import './App.css'

// ─── Epoch Player ─────────────────────────────────────────────────────────────

interface EpochPlayerProps {
  history: EpochSnapshot[]
  runId: number
}

function EpochPlayer({ history, runId }: EpochPlayerProps) {
  const [idx, setIdx]         = useState(0)
  const [playing, setPlaying] = useState(false)
  const timerRef              = useRef<number>(0)
  const n                     = history.length

  const maxErr = Math.max(...history.map(h => h.error), 1)
  const allW   = history.flatMap(h => [h.w1, h.w2])
  const wMin   = Math.min(...allW)
  const wMax   = Math.max(...allW)
  const allB   = history.map(h => h.bias)
  const bMin   = Math.min(...allB)
  const bMax   = Math.max(...allB)

  const clamp01 = (v: number, lo: number, hi: number): number =>
    hi === lo ? 0.5 : Math.max(0, Math.min(1, (v - lo) / (hi - lo)))

  const lerpColor = (t: number): string => {
    const r = Math.round(t * 220)
    const g = Math.round(188 - t * 118)
    const b = Math.round(188 - t * 118)
    return `rgb(${r},${g},${b})`
  }

  const stop = useCallback(() => {
    setPlaying(false)
    clearInterval(timerRef.current)
  }, [])

  const play = useCallback(() => {
    clearInterval(timerRef.current)
    setIdx(0)
    setPlaying(true)
  }, [])

  // auto-play on new training run
  useEffect(() => {
    play()
    return () => clearInterval(timerRef.current)
  // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [runId])

  // advance frame
  useEffect(() => {
    if (!playing) return
    timerRef.current = setInterval(() => {
      setIdx(prev => {
        if (prev >= n - 1) {
          setPlaying(false)
          clearInterval(timerRef.current)
          return prev
        }
        return prev + 1
      })
    }, 80)
    return () => clearInterval(timerRef.current)
  }, [playing, n])

  const safeIdx = Math.min(idx, n - 1)
  const h       = history[safeIdx]

  const errColor =
    h.error === 0 ? '#3cd680' : h.error <= 1 ? '#f0c840' : '#dc4646'

  const wAbs = Math.max(Math.abs(wMin), Math.abs(wMax))
  const bAbs = Math.max(Math.abs(bMin), Math.abs(bMax))

  const params = [
    { key: 'w1',   label: 'W₁  (peso A)', value: h.w1,   pct: clamp01(h.w1,   wMin, wMax), color: lerpColor(clamp01(Math.abs(h.w1),   0, wAbs)) },
    { key: 'w2',   label: 'W₂  (peso B)', value: h.w2,   pct: clamp01(h.w2,   wMin, wMax), color: lerpColor(clamp01(Math.abs(h.w2),   0, wAbs)) },
    { key: 'bias', label: 'Bias',          value: h.bias, pct: clamp01(h.bias, bMin, bMax), color: lerpColor(clamp01(Math.abs(h.bias), 0, bAbs)) },
  ]

  return (
    <div className="epoch-player">
      <div className="ep-header">
        <span className="ep-title">⚡ Evolução dos Pesos por Época</span>
        <span className="epoch-badge">época {h.epoch}</span>
      </div>

      <input
        type="range"
        min={0}
        max={n - 1}
        value={idx}
        className="epoch-slider"
        onChange={e => { stop(); setIdx(Number(e.target.value)) }}
      />

      <div className="param-grid">
        {params.map(p => (
          <div key={p.key} className="param-card">
            <div className="param-label">{p.label}</div>
            <div className="param-value">{p.value.toFixed(4)}</div>
            <div className="param-bar-bg">
              <div className="param-bar" style={{ width: `${p.pct * 100}%`, background: p.color }} />
            </div>
          </div>
        ))}
      </div>

      <div className="error-row">
        <div className="error-info">
          <div className="param-label">Erro Total</div>
          <div className="error-value" style={{ color: errColor }}>{h.error}</div>
        </div>
        <div className="error-bar-bg">
          <div className="error-bar" style={{ width: `${(h.error / maxErr) * 100}%`, background: errColor }} />
        </div>
        <span className="status-icon">
          {h.error === 0 ? '✅' : h.error <= 1 ? '🟡' : '❌'}
        </span>
      </div>

      <div className="ep-buttons">
        <button className="btn-stop" onClick={stop}>⏹ Parar</button>
        <button className="btn-play" onClick={play}>▶ Replay</button>
      </div>
    </div>
  )
}

// ─── Error Chart (SVG animado) ────────────────────────────────────────────────

interface ErrorChartProps {
  history: EpochSnapshot[]
  runId: number
}

function ErrorChart({ history, runId }: ErrorChartProps) {
  const W   = 640
  const H   = 220
  const pad = { top: 20, right: 20, bottom: 40, left: 50 }
  const cw  = W - pad.left - pad.right
  const ch  = H - pad.top  - pad.bottom
  const n   = history.length

  const errors = history.map(h => h.error)
  const maxE   = Math.max(...errors, 1)

  const px = (i: number) => pad.left + (n > 1 ? (i / (n - 1)) * cw : 0)
  const py = (e: number) => pad.top  + ch - (e / maxE) * ch

  const pts     = errors.map((e, i) => `${px(i).toFixed(1)},${py(e).toFixed(1)}`).join(' ')
  const areaPts = `${px(0).toFixed(1)},${(pad.top + ch).toFixed(1)} ${pts} ${px(n - 1).toFixed(1)},${(pad.top + ch).toFixed(1)}`

  const gridLines = [0, 1, 2, 3, 4].map(g => ({
    gy:  pad.top + ch * g / 4,
    val: maxE * (4 - g) / 4,
  }))

  return (
    <svg
      key={runId}
      xmlns="http://www.w3.org/2000/svg"
      viewBox={`0 0 ${W} ${H}`}
      className="error-chart"
    >
      <defs>
        <linearGradient id="ag" x1="0" y1="0" x2="0" y2="1">
          <stop offset="0%"   stopColor="#00bcbc" stopOpacity="0.35" />
          <stop offset="100%" stopColor="#00bcbc" stopOpacity="0"    />
        </linearGradient>
      </defs>

      {gridLines.map(({ gy, val }, i) => (
        <g key={i}>
          <line x1={pad.left} y1={gy} x2={W - pad.right} y2={gy} stroke="#2a2a40" strokeWidth="1" />
          <text x={pad.left - 6} y={gy + 4} fill="#888" fontSize="11" textAnchor="end">
            {val.toFixed(1)}
          </text>
        </g>
      ))}

      <line x1={pad.left} y1={pad.top}      x2={pad.left}      y2={pad.top + ch} stroke="#444" strokeWidth="1.5" />
      <line x1={pad.left} y1={pad.top + ch} x2={W - pad.right} y2={pad.top + ch} stroke="#444" strokeWidth="1.5" />

      <text x={W / 2} y={H - 4}               fill="#666" fontSize="11" textAnchor="middle">Épocas</text>
      <text x={12}    y={pad.top + ch / 2}     fill="#666" fontSize="11" textAnchor="middle"
            transform={`rotate(-90,12,${pad.top + ch / 2})`}>Erro</text>

      <polygon points={areaPts} fill="url(#ag)" />
      <polyline
        points={pts}
        fill="none"
        stroke="#00bcbc"
        strokeWidth="2.5"
        strokeLinejoin="round"
        strokeLinecap="round"
        className="chart-line"
      />
    </svg>
  )
}

// ─── Truth Table ──────────────────────────────────────────────────────────────

interface TruthTableProps {
  testResults: TestResult[]
  converged: boolean
  finalW1: number
  finalW2: number
  finalBias: number
  finalError: number
}

function TruthTable({ testResults, converged, finalW1, finalW2, finalBias, finalError }: TruthTableProps) {
  return (
    <div className="truth-section">
      <table className="truth-table">
        <thead>
          <tr><th>A</th><th>B</th><th>Esperado</th><th>Previsto</th><th></th></tr>
        </thead>
        <tbody>
          {testResults.map((r, i) => (
            <tr key={i} className={r.predicted !== r.expected ? 'row-wrong' : ''}>
              <td>{r.A}</td>
              <td>{r.B}</td>
              <td>{r.expected}</td>
              <td>{r.predicted}</td>
              <td>{r.predicted === r.expected ? '✅' : '❌'}</td>
            </tr>
          ))}
        </tbody>
      </table>

      <div className="final-weights">
        <div className="fw-title">Pesos Finais</div>
        <table className="fw-table">
          <tbody>
            <tr><td>W₁</td>        <td className="fw-val">{finalW1}</td></tr>
            <tr><td>W₂</td>        <td className="fw-val">{finalW2}</td></tr>
            <tr><td>Bias</td>      <td className="fw-val">{finalBias}</td></tr>
            <tr><td>Erro Final</td><td className="fw-val">{finalError}</td></tr>
          </tbody>
        </table>
      </div>

      <div className={`convergence-badge ${converged ? 'ok' : 'fail'}`}>
        {converged ? '✅ Convergiu!' : '⚠️ Não convergiu — XOR não é linearmente separável'}
      </div>
    </div>
  )
}

// ─── App ──────────────────────────────────────────────────────────────────────

export default function App() {
  const [gate,   setGate]   = useState<GateName>('AND')
  const [epochs, setEpochs] = useState(50)
  const [lr,     setLr]     = useState(0.1)
  const [result, setResult] = useState<TrainResult | null>(null)
  const [runId,  setRunId]  = useState(0)

  const handleTrain = useCallback(() => {
    const r = trainPerceptron(gate, epochs, lr)
    setResult(r)
    setRunId(id => id + 1)
  }, [gate, epochs, lr])

  // treinar ao montar
  useEffect(() => { handleTrain() }, [])   // eslint-disable-line react-hooks/exhaustive-deps

  return (
    <div className="app">
      <header className="app-header">
        <h1>⚡ Perceptron — Portas Lógicas</h1>
        <p>
          Selecione uma porta lógica, ajuste os parâmetros e treine o Perceptron.<br />
          A animação mostra a evolução dos pesos a cada época.
        </p>
      </header>

      {/* ── Controles ── */}
      <section className="controls">
        <div className="gate-row">
          {(Object.keys(GATES) as GateName[]).map(g => (
            <button
              key={g}
              className={`gate-btn ${gate === g ? 'active' : ''}`}
              onClick={() => setGate(g)}
            >
              {g}
            </button>
          ))}
        </div>

        <div className="slider-row">
          <label className="slider-label">
            <span>Épocas <strong>{epochs}</strong></span>
            <input
              type="range" min={10} max={200} step={10} value={epochs}
              onChange={e => setEpochs(Number(e.target.value))}
            />
          </label>
          <label className="slider-label">
            <span>Taxa de Aprendizado <strong>{lr.toFixed(2)}</strong></span>
            <input
              type="range" min={0.01} max={0.5} step={0.01} value={lr}
              onChange={e => setLr(Number(e.target.value))}
            />
          </label>
        </div>

        <button className="train-btn" onClick={handleTrain}>
          ▶ Treinar Perceptron
        </button>
      </section>

      {/* ── Resultados ── */}
      {result && (
        <section className="results">
          <div className="results-grid">
            <div>
              <h2 className="section-title">📋 Tabela Verdade</h2>
              <TruthTable
                testResults={result.testResults}
                converged={result.converged}
                finalW1={result.finalW1}
                finalW2={result.finalW2}
                finalBias={result.finalBias}
                finalError={result.finalError}
              />
            </div>

            <div>
              <h2 className="section-title">🎬 Animação dos Pesos</h2>
              <EpochPlayer history={result.history} runId={runId} />
            </div>
          </div>

          <h2 className="section-title">📉 Erro por Época</h2>
          <ErrorChart history={result.history} runId={runId} />
        </section>
      )}
    </div>
  )
}
