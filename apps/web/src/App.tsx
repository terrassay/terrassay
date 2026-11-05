import { useEffect, useState } from 'react'
import { api, type Issue, type Project, type ProjectSummary } from './api'

const MAP_URL = import.meta.env.VITE_MAP_URL ?? 'https://web.geolibre.app/?maponly&theme=light'

function Stat({ label, value, accent }: { label: string; value: number; accent?: boolean }) {
  return (
    <div className={`stat${accent ? ' stat-accent' : ''}`}>
      <span className="stat-value">{value}</span>
      <span className="stat-label">{label}</span>
    </div>
  )
}

function IssueRow({ issue }: { issue: Issue }) {
  return (
    <li className="issue-row">
      <span className={`priority priority-${issue.priority}`}>{issue.priority}</span>
      <div className="issue-copy">
        <strong>{issue.title}</strong>
        <span>
          {issue.externalId}
          {issue.discipline ? ` · ${issue.discipline}` : ''}
          {issue.assigneeName ? ` · ${issue.assigneeName}` : ''}
        </span>
      </div>
      <span className={`status status-${issue.status}`}>{issue.status.replace('_', ' ')}</span>
    </li>
  )
}

export default function App() {
  const [project, setProject] = useState<Project | null>(null)
  const [summary, setSummary] = useState<ProjectSummary | null>(null)
  const [issues, setIssues] = useState<Issue[]>([])
  const [error, setError] = useState<string | null>(null)

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
        const [projectSummary, projectIssues] = await Promise.all([
          api.summary(first.id),
          api.issues(first.id),
        ])
        if (cancelled) return
        setSummary(projectSummary)
        setIssues(projectIssues)
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
          <span className="phase">Phase 0</span>
          <span className="demo-badge">Synthetic demo</span>
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

          <div className="divider" />

          <p className="eyebrow">Layers</p>
          <ul className="layer-list">
            <li><span className="swatch swatch-site" /> Site boundary</li>
            <li><span className="swatch swatch-flood" /> Flood hazard</li>
            <li><span className="swatch swatch-terrain" /> Hillshade</li>
            <li><span className="swatch swatch-issues" /> Issues</li>
          </ul>

          <div className="divider" />

          <p className="eyebrow">Location</p>
          <p className="mono">47.3845° N, 8.5170° E</p>
          <p className="muted small">Verified · 3 control points · RMS 0.18 m</p>
        </aside>

        <main className="map-area">
          <iframe
            className="map-frame"
            title="Terrassay map"
            src={MAP_URL}
            allow="fullscreen; geolocation"
          />
          <div className="map-caption">
            <span>Hafenquartier Zürich</span>
            <span>GeoLibre · MapLibre</span>
          </div>
        </main>

        <aside className="panel issues-panel">
          <div className="panel-heading">
            <p className="eyebrow">Coordination</p>
            <span className="count">{summary?.issueCount ?? 0} issues</span>
          </div>

          {error && <p className="error">{error}</p>}

          {!error && issues.length === 0 && <p className="muted">No issues loaded yet.</p>}

          <ul className="issue-list">
            {issues.map((issue) => (
              <IssueRow key={issue.id} issue={issue} />
            ))}
          </ul>

          <div className="divider" />
          <p className="eyebrow">Assistant</p>
          <div className="assistant-stub">
            <p>Ask: “Which critical issues are inside the flood zone?”</p>
            <button type="button" disabled>
              Phase 1
            </button>
          </div>
        </aside>
      </div>
    </div>
  )
}
