import { useEffect, useRef, useState } from 'react'
import { connect, type GeoLibreEmbedClient } from '@geolibre/embed'
import { api } from './api'

const MAP_URL = import.meta.env.VITE_MAP_URL ?? 'http://localhost:8081/'

type Props = {
  projectId: string
  onReady: (client: GeoLibreEmbedClient) => void
  onSelection: (featureIds: string[]) => void
}

type LayerDefinition = {
  id: string
  name: string
  url: string
}

const LAYERS: LayerDefinition[] = [
  { id: 'terrassay:site', name: 'Site boundary', url: 'site' },
  { id: 'terrassay:flood', name: 'Flood hazard', url: 'flood' },
  { id: 'terrassay:buildings', name: 'Buildings', url: 'buildings' },
]

export default function MapPanel({ projectId, onReady, onSelection }: Props) {
  const iframeRef = useRef<HTMLIFrameElement>(null)
  const clientRef = useRef<GeoLibreEmbedClient | null>(null)
  const [error, setError] = useState<string | null>(null)
  const [ready, setReady] = useState(false)

  useEffect(() => {
    let cancelled = false

    async function boot() {
      const iframe = iframeRef.current
      if (!iframe) return
      try {
        const map = await connect(iframe, {
          origin: new URL(MAP_URL).origin,
          timeoutMs: 20000,
        })
        if (cancelled) {
          map.disconnect()
          return
        }
        clientRef.current = map
        setReady(true)
        onReady(map)

        await map.setView({ bbox: [8.514, 47.382, 8.520, 47.387] })

        for (const layer of LAYERS) {
          try {
            const response = await fetch(api.featureUrl(projectId, layer.url))
            if (!response.ok) throw new Error(`${response.status}`)
            const geojson = (await response.json()) as Record<string, unknown>
            await map.addLayer({
              id: layer.id,
              name: layer.name,
              type: 'geojson',
              source: { type: 'geojson', data: geojson },
              geojson,
              style:
                layer.url === 'flood'
                  ? { fillColor: '#4f8cff', fillOpacity: 0.22, lineColor: '#4f8cff' }
                  : layer.url === 'buildings'
                    ? { fillColor: '#7aa2f7', fillOpacity: 0.3, lineColor: '#9bb7ff' }
                    : { fillColor: '#f2a65a', fillOpacity: 0.06, lineColor: '#f2a65a' },
            })
          } catch (cause) {
            setError(`Layer ${layer.name} failed: ${cause instanceof Error ? cause.message : 'unknown'}`)
          }
        }

        try {
          const issues = await fetch(api.issuesGeoJsonUrl(projectId))
          if (issues.ok) {
            const geojson = (await issues.json()) as Record<string, unknown>
            await map.addLayer({
              id: 'terrassay:issues',
              name: 'Issues',
              type: 'geojson',
              source: { type: 'geojson', data: geojson },
              geojson,
              style: { circleColor: '#ff6b6b', circleRadius: 4, circleStrokeColor: '#141a21' },
            })
          }
        } catch (cause) {
          setError(`Issues layer failed: ${cause instanceof Error ? cause.message : 'unknown'}`)
        }

        map.on('selectionChanged', ({ featureIds }) => {
          onSelection(featureIds)
        })
      } catch (cause) {
        setError(
          `Map bridge unavailable (${cause instanceof Error ? cause.message : 'unknown'}). The shell and API still work.`,
        )
      }
    }

    void boot()

    return () => {
      cancelled = true
      clientRef.current?.disconnect()
      clientRef.current = null
    }
  }, [projectId, onReady, onSelection])

  return (
    <div className="map-wrap">
      <iframe
        ref={iframeRef}
        className="map-frame"
        title="Terrassay map"
        src={MAP_URL}
        allow="fullscreen; geolocation"
      />
      <div className="map-overlay">
        <span>{ready ? 'Map bridge connected' : 'Connecting map…'}</span>
        <span>GeoLibre</span>
      </div>
      {error && <div className="map-error">{error}</div>}
    </div>
  )
}
