# Map host

Phase 0 uses the official GeoLibre image as the map host:

```yaml
map:
  image: ghcr.io/opengeos/geolibre:latest
  environment:
    GEOLIBRE_EMBED_ORIGINS: "http://localhost:8080,http://localhost:5173"
```

The web shell embeds it directly at `http://localhost:8081/` in Compose
(Phase 0). A same-origin `/map/` subpath needs a GeoLibre base-path build and
is deferred until the embed spike. During local Vite development the public
`https://web.geolibre.app/` may be used as a visual fallback.

## Phase 0 spike checklist

- [ ] Map iframe renders in the shell.
- [ ] `GEOLIBRE_EMBED_ORIGINS` accepts the shell origin.
- [ ] `@geolibre/embed` can connect and call `setView`.
- [ ] `addLayer` accepts a signed GeoJSON URL.
- [ ] `setFilter` and `highlight` work on the issue layer.
- [ ] `selectionChanged` reaches the shell.

Do not fork GeoLibre for Phase 0. If a command is missing, add a first-party
plugin inside the map host and document the gap.
