import { useCallback, useEffect, useRef, useState } from 'react'
import type { GeoLibreEmbedClient } from '@geolibre/embed'
import {
  api,
  type AskResponse,
  type Issue,
  type LayerInfo,
  type Localization,
  type Project,
  type ProjectSummary,
} from './api'
import MapPanel from './MapPanel'

const LAYER_TOGGLES = [
  { id: 'terrassay:site', label: 'Site boundary' },
  { id: 'terrassay:flood', label: 'Flood hazard' },
  { id: 'terrassay:buildings', label: 'Buildings' },
  { id: 'terrassay:issues', label: 'Issues' },
]

function Stat({ label, value, accent }: { label: string; value: number; accent?: boolean }) {
  return (
    <div className={`stat${accent ? ' stat-accent' : ''}`}>
      <span className="stat-value">{value}</span>
      <span className="stat-label">{label}</span>
    </div>
  )
}

function IssueRow({
  issue,
  selected,
  onClick,
}: {
  issue: Issue
  selected: boolean
  onClick: () => void
}) {
  return (
    <li>
      <button
        type="button"
        className={`issue-row${selected ? ' issue-row-selected' : ''}`}
        onClick={onClick}
      >
        <span className={`priority priority-${issue.priority}`}>{issue.priority}</span>
        <span className="issue-copy">
          <strong>{issue.title}</strong>
          <span>
            {issue.externalId}
            {issue.discipline ? ` · ${issue.discipline}` : ''}
            {issue.assigneeName ? ` · ${issue.assigneeName}` : ''}
          </span>
        </span>
        <span className={`status status-${issue.status}`}>{issue.status.replace('_', ' ')}</span>
      </button>
    </li>
  )
}

export default function App() {
  const [project, setProject] = useState<Project | null>(null)
  const [summary, setSummary] = useState<ProjectSummary | null>(null)
  const [issues, setIssues] = useState<Issue[]>([])
  const [localization, setLocalization] = useState<Localization | null>(null)
  const [layers, setLayers] = useState<LayerInfo[]>([])
  const [visibleLayers, setVisibleLayers] = useState<Record<string, boolean>>({
    'terrassay:site': true,
    'terrassay:flood': true,
    'terrassay:buildings': true,
    'terrassay:issues': true,
  })
  const [selectedIssueId, setSelectedIssueId] = useState<string | null>(null)
  const [question, setQuestion] = useState('Which critical issues are inside the flood zone?')
  const [answer, setAnswer] = useState<AskResponse | null>(null)
  const [asking, setAsking] = useState(false)
  const [syncStatus, setSyncStatus] = useState<string | null>(null)
  const [mapReady, setMapReady] = useState(false)
  const [error, setError] = useState<string | null>(null)
  const mapRef = useRef<GeoLibreEmbedClient | null>(null)

  useEffect(() => {
    let cancelled = false
    async function load() {
      try {
        const projects = await api.projects()
        if (cancelled) return
        const first = projects[0]
        if (!first) {
          setError('No seeded project found. Run docker compose up to initialize the demo data.')
          return
        }
        setProject(first)
        const [projectSummary, projectIssues, projectLocalization, projectLayers] =
          await Promise.all([
            api.summary(first.id),
            api.issues(first.id),
            api.localization(first.id),
            api.layers(first.id),
          ])
        if (cancelled) return
        setSummary(projectSummary)
        setIssues(projectIssues)
        setLocalization(projectLocalization)
        setLayers(projectLayers)
      } catch (cause) {
        if (!cancelled) {
          setError(
            `API unavailable (${cause instanceof Error ? cause.message : 'unknown error'}). Start the API or run docker compose up.`,
          )
        }
      }
    }
    void load()
    return () => {
      cancelled = true
    }
  }, [])

  const handleMapReady = useCallback((client: GeoLibreEmbedClient) => {
    mapRef.current = client
    setMapReady(true)
  }, [])

  const handleSelection = useCallback((featureIds: string[]) => {
    if (featureIds.length > 0) setSelectedIssueId(featureIds[0])
  }, [])

  function focusIssue(issue: Issue) {
    setSelectedIssueId(issue.id)
    if (issue.lon != null && issue.lat != null) {
      void mapRef.current?.setView({ center: [issue.lon, issue.lat], zoom: 17, duration: 600 })
    }
    void mapRef.current?.highlightFeature({
      layerId: 'terrassay:issues',
      featureId: issue.id,
    })
  }

  function toggleLayer(layerId: string) {
    const next = !visibleLayers[layerId]
    setVisibleLayers((current) => ({ ...current, [layerId]: next }))
    void mapRef.current?.setLayerVisibility(layerId, next)
  }

  async function ask() {
    if (!project || !question.trim()) return
    setAsking(true)
    try {
      const response = await api.ask(project.id, question.trim())
      setAnswer(response)
      if (response.mapAction?.type === 'highlight') {
        void mapRef.current?.highlightFeature({
          layerId: response.mapAction.layerId,
          featureIds: response.mapAction.featureIds,
          fit: true,
        })
      }
    } catch (cause) {
      setError(`Assistant failed: ${cause instanceof Error ? cause.message : 'unknown error'}`)
    } finally {
      setAsking(false)
    }
  }

  async function sync() {
    if (!project) return
    setSyncStatus('Queuing mock sync…')
    try {
      const job = await api.sync(project.id)
      setSyncStatus(`Queued ${job.status} · ${job.jobId.slice(0, 8)}`)
      setTimeout(async () => {
        const next = await api.summary(project.id)
        setSummary(next)
      }, 2200)
    } catch (cause) {
      setSyncStatus(`Sync failed: ${cause instanceof Error ? cause.message : 'unknown error'}`)
    }
  }

  return (
    <div className="shell">
      <header className="topbar">
        <div className="brand">
          <span className="brand-mark">T</span>
          <div>
            <h1>Terrassay</h1>
            <p>Physical asset intelligence, grounded.</p>
          </div>
        </div>
        <div className="topbar-meta">
          <span className="phase">Phase 1</span>
          <span className="demo-badge">Synthetic demo</span>
          <button type="button" className="sync-button" onClick={() => void sync()}>
            Sync mock
          </button>
        </div>
      </header>

      <div className="workspace">
        <aside className="panel project-panel">
          <p className="eyebrow">Project</p>
          <h2>{project?.name ?? 'Loading project…'}</h2>
          <p className="muted">
            {project?.code ?? '—'} · {project?.analysisCrs ?? '—'}
          </p>

          {summary && (
            <div className="stats">
              <Stat label="Issues" value={summary.issueCount} />
              <Stat label="Critical" value={summary.criticalCount} accent />
              <Stat label="Clashes" value={summary.clashCount} />
              <Stat label="Sheets" value={summary.sheetCount} />
            </div>
          )}

          {syncStatus && <p className="sync-status">{syncStatus}</p>}

          <div className="divider" />

          <p className="eyebrow">Layers</p>
          <ul className="layer-list">
            {LAYER_TOGGLES.map((layer) => (
              <li key={layer.id}>
                <label className="layer-toggle">
                  <input
                    type="checkbox"
                    checked={visibleLayers[layer.id] ?? true}
                    onChange={() => toggleLayer(layer.id)}
                  />
                  <span className={`swatch swatch-${layer.id.split(':')[1]}`} />
                  {layer.label}
                </label>
              </li>
            ))}
          </ul>
          <p className="muted small">{layers.length} layers registered</p>

          <div className="divider" />

          <p className="eyebrow">Location</p>
          <p className="mono">47.3845° N, 8.5170° E</p>
          {localization ? (
            <p className="muted small">
              {localization.confidence} · {localization.method.replace('_', ' ')} ·{' '}
              {localization.accuracyM != null ? `${localization.accuracyM.toFixed(2)} m RMS` : 'accuracy n/a'}
            </p>
          ) : (
            <p className="muted small">No localization record</p>
          )}
        </aside>

        <main className="map-area">
          {project ? (
            <MapPanel
              projectId={project.id}
              onReady={handleMapReady}
              onSelection={handleSelection}
            />
          ) : (
            <div className="map-placeholder">Waiting for project…</div>
          )}
          <div className="map-caption">
            <span>Hafenquartier Zürich</span>
            <span>{mapReady ? 'Map bridge connected' : 'GeoLibre'}</span>
          </div>
        </main>

        <aside className="panel issues-panel">
          <div className="panel-heading">
            <p className="eyebrow">Coordination</p>
            <span className="count">{summary?.issueCount ?? 0} issues</span>
          </div>

          {error && <p className="error">{error}</p>}

          <ul className="issue-list">
            {issues.map((issue) => (
              <IssueRow
                key={issue.id}
                issue={issue}
                selected={issue.id === selectedIssueId}
                onClick={() => focusIssue(issue)}
              />
            ))}
          </ul>

          <div className="divider" />
          <p className="eyebrow">Assistant</p>
          <form
            className="assistant"
            onSubmit={(event) => {
              event.preventDefault()
              void ask()
            }}
          >
            <textarea
              value={question}
              onChange={(event) => setQuestion(event.target.value)}
              rows={2}
              placeholder="Ask about the project…"
            />
            <button type="submit" disabled={asking || !project}>
              {asking ? 'Thinking…' : 'Ask'}
            </button>
          </form>

          {answer && (
            <div className="answer">
              <p>{answer.answer}</p>
              <div className="citations">
                {answer.citations.slice(0, 4).map((citation) => (
                  <span key={`${citation.kind}-${citation.id}`} className="citation">
                    {citation.label}
                  </span>
                ))}
                {answer.citations.length > 4 && (
                  <span className="citation">+{answer.citations.length - 4} more</span>
                )}
              </div>
              <span className="answer-mode">{answer.mode}</span>
            </div>
          )}
        </aside>
      </div>
    </div>
  )
}
