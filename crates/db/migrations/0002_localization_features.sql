CREATE TABLE IF NOT EXISTS project_localizations (
    project_id uuid PRIMARY KEY REFERENCES projects(id) ON DELETE CASCADE,
    method text NOT NULL,
    source_crs text NOT NULL,
    target_crs text NOT NULL,
    transform jsonb NOT NULL DEFAULT '{}'::jsonb,
    accuracy_m double precision,
    confidence text NOT NULL CHECK (confidence IN ('verified', 'calculated', 'indicative')),
    verified boolean NOT NULL DEFAULT false,
    notes text,
    updated_at timestamptz NOT NULL DEFAULT now()
);

CREATE TABLE IF NOT EXISTS project_features (
    id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
    project_id uuid NOT NULL REFERENCES projects(id) ON DELETE CASCADE,
    layer_key text NOT NULL,
    properties jsonb NOT NULL DEFAULT '{}'::jsonb,
    geometry geometry(Geometry, 4326) NOT NULL
);

CREATE INDEX IF NOT EXISTS project_features_project_key_idx
    ON project_features (project_id, layer_key);
CREATE INDEX IF NOT EXISTS project_features_geometry_idx
    ON project_features USING gist (geometry);

ALTER TABLE layers ADD COLUMN IF NOT EXISTS layer_key text;
