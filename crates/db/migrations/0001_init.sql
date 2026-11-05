CREATE EXTENSION IF NOT EXISTS postgis;
CREATE EXTENSION IF NOT EXISTS pgcrypto;

CREATE TABLE IF NOT EXISTS projects (
    id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
    name text NOT NULL,
    code text,
    status text NOT NULL DEFAULT 'active',
    analysis_crs text NOT NULL DEFAULT 'EPSG:3857',
    bbox geometry(Polygon, 4326),
    last_synced_at timestamptz,
    created_at timestamptz NOT NULL DEFAULT now(),
    updated_at timestamptz NOT NULL DEFAULT now()
);

CREATE TABLE IF NOT EXISTS issues (
    id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
    project_id uuid NOT NULL REFERENCES projects(id) ON DELETE CASCADE,
    external_id text NOT NULL,
    title text NOT NULL,
    status text NOT NULL DEFAULT 'open',
    priority text NOT NULL DEFAULT 'normal'
        CHECK (priority IN ('critical', 'high', 'normal', 'low')),
    discipline text,
    assignee_name text,
    due_date date,
    location geometry(PointZ, 4326),
    source_updated_at timestamptz,
    synced_at timestamptz NOT NULL DEFAULT now(),
    UNIQUE (project_id, external_id)
);

CREATE INDEX IF NOT EXISTS issues_project_priority_idx ON issues (project_id, priority);
CREATE INDEX IF NOT EXISTS issues_location_idx ON issues USING gist (location);

CREATE TABLE IF NOT EXISTS clashes (
    id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
    project_id uuid NOT NULL REFERENCES projects(id) ON DELETE CASCADE,
    external_id text NOT NULL,
    title text NOT NULL,
    status text NOT NULL DEFAULT 'open',
    priority text NOT NULL DEFAULT 'normal',
    location geometry(PointZ, 4326),
    synced_at timestamptz NOT NULL DEFAULT now(),
    UNIQUE (project_id, external_id)
);

CREATE INDEX IF NOT EXISTS clashes_location_idx ON clashes USING gist (location);

CREATE TABLE IF NOT EXISTS sheets (
    id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
    project_id uuid NOT NULL REFERENCES projects(id) ON DELETE CASCADE,
    external_id text NOT NULL,
    number text NOT NULL,
    title text NOT NULL,
    discipline text,
    synced_at timestamptz NOT NULL DEFAULT now(),
    UNIQUE (project_id, external_id)
);

CREATE TABLE IF NOT EXISTS datasets (
    id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
    project_id uuid NOT NULL REFERENCES projects(id) ON DELETE CASCADE,
    name text NOT NULL,
    kind text NOT NULL CHECK (kind IN ('vector', 'raster', 'pointcloud', 'tiles3d', 'table')),
    format text NOT NULL,
    source_kind text NOT NULL CHECK (source_kind IN ('upload', 'url', 'connector', 'analysis')),
    crs text,
    bbox geometry(Polygon, 4326),
    licence text,
    attribution text,
    created_at timestamptz NOT NULL DEFAULT now()
);

CREATE TABLE IF NOT EXISTS layers (
    id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
    project_id uuid NOT NULL REFERENCES projects(id) ON DELETE CASCADE,
    dataset_id uuid REFERENCES datasets(id) ON DELETE SET NULL,
    name text NOT NULL,
    type text NOT NULL CHECK (type IN ('geojson', 'mvt', 'raster', 'cog', 'xyz', 'pmtiles', '3dtiles', 'pointcloud')),
    style jsonb NOT NULL DEFAULT '{}'::jsonb,
    visible boolean NOT NULL DEFAULT true,
    opacity double precision NOT NULL DEFAULT 1 CHECK (opacity >= 0 AND opacity <= 1),
    z_index integer NOT NULL DEFAULT 0,
    licence text,
    attribution text,
    source_config jsonb NOT NULL DEFAULT '{}'::jsonb,
    created_at timestamptz NOT NULL DEFAULT now()
);

CREATE TABLE IF NOT EXISTS jobs (
    id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
    project_id uuid REFERENCES projects(id) ON DELETE CASCADE,
    type text NOT NULL,
    status text NOT NULL DEFAULT 'queued'
        CHECK (status IN ('queued', 'running', 'succeeded', 'failed', 'cancelled')),
    progress integer NOT NULL DEFAULT 0 CHECK (progress BETWEEN 0 AND 100),
    payload jsonb NOT NULL DEFAULT '{}'::jsonb,
    error jsonb,
    created_at timestamptz NOT NULL DEFAULT now(),
    updated_at timestamptz NOT NULL DEFAULT now()
);

CREATE TABLE IF NOT EXISTS audit_events (
    id bigint GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    project_id uuid REFERENCES projects(id) ON DELETE CASCADE,
    action text NOT NULL,
    resource_type text NOT NULL,
    resource_id text,
    metadata jsonb NOT NULL DEFAULT '{}'::jsonb,
    created_at timestamptz NOT NULL DEFAULT now()
);
