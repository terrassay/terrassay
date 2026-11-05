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
}

async function getJSON<T>(path: string): Promise<T> {
  const response = await fetch(path, { headers: { accept: 'application/json' } })
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
}
