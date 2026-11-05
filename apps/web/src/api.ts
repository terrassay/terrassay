export type Project = {
  id: string
  name: string
  code: string | null
  status: string
  analysisCrs: string
  createdAt: string
}

export type ProjectSummary = {
  projectId: string
  issueCount: number
  criticalCount: number
  clashCount: number
  sheetCount: number
  lastSyncedAt: string | null
}

export type Issue = {
  id: string
  externalId: string
  title: string
  status: string
  priority: 'critical' | 'high' | 'normal' | 'low'
  discipline: string | null
  assigneeName: string | null
  lon: number | null
  lat: number | null
}

export type Localization = {
  method: string
  sourceCrs: string
  targetCrs: string
  transform: Record<string, unknown>
  accuracyM: number | null
  confidence: 'verified' | 'calculated' | 'indicative'
  verified: boolean
  updatedAt: string
}

export type LayerInfo = {
  id: string
  name: string
  layerType: string
  visible: boolean
  opacity: number
  layerKey: string | null
  licence: string | null
  attribution: string | null
}

export type Citation = {
  kind: string
  id: string
  label: string
}

export type AskResponse = {
  answer: string
  citations: Citation[]
  mapAction: {
    type: 'highlight'
    layerId: string
    featureIds: string[]
  } | null
  mode: string
}

async function getJSON<T>(path: string): Promise<T> {
  const response = await fetch(path, { headers: { accept: 'application/json' } })
  if (!response.ok) {
    throw new Error(`${response.status} ${response.statusText}`)
  }
  return (await response.json()) as T
}

async function postJSON<T>(path: string, body: unknown): Promise<T> {
  const response = await fetch(path, {
    method: 'POST',
    headers: { 'content-type': 'application/json', accept: 'application/json' },
    body: JSON.stringify(body),
  })
  if (!response.ok) {
    throw new Error(`${response.status} ${response.statusText}`)
  }
  return (await response.json()) as T
}

export const api = {
  projects: () => getJSON<Project[]>('/api/v1/projects'),
  summary: (projectId: string) =>
    getJSON<ProjectSummary>(`/api/v1/projects/${projectId}/summary`),
  issues: (projectId: string, limit = 12) =>
    getJSON<Issue[]>(`/api/v1/projects/${projectId}/issues?limit=${limit}`),
  localization: (projectId: string) =>
    getJSON<Localization | null>(`/api/v1/projects/${projectId}/localization`),
  layers: (projectId: string) =>
    getJSON<LayerInfo[]>(`/api/v1/projects/${projectId}/layers`),
  ask: (projectId: string, question: string) =>
    postJSON<AskResponse>(`/api/v1/projects/${projectId}/assistant/ask`, { question }),
  sync: (projectId: string) =>
    postJSON<{ jobId: string; status: string }>(`/api/v1/projects/${projectId}/sync`, {}),
  featureUrl: (projectId: string, layerKey: string) =>
    `/api/v1/projects/${projectId}/features/${layerKey}`,
  issuesGeoJsonUrl: (projectId: string) =>
    `/api/v1/projects/${projectId}/issues.geojson`,
}
