-- Synthetic demo project. No real Revizto, customer, or personal data.
INSERT INTO projects (id, name, code, status, analysis_crs, bbox, last_synced_at)
VALUES (
  '018f0000-0000-7000-8000-000000000001',
  'Hafenquartier Zürich',
  'HQZ-2026',
  'active',
  'EPSG:2056',
  ST_MakeEnvelope(8.5140, 47.3820, 8.5200, 47.3870, 4326),
  now() - interval '12 minutes'
)
ON CONFLICT (id) DO NOTHING;

INSERT INTO issues (
  project_id, external_id, title, status, priority, discipline, assignee_name, due_date, location
)
SELECT
  '018f0000-0000-7000-8000-000000000001',
  'ISSUE-' || lpad(i::text, 3, '0'),
  (ARRAY['Facade', 'MEP', 'Structure', 'Landscape'])[1 + ((i - 1) % 4)]
    || ' coordination item ' || i,
  CASE
    WHEN i % 5 = 0 THEN 'closed'
    WHEN i % 3 = 0 THEN 'in_progress'
    ELSE 'open'
  END,
  CASE
    WHEN i <= 60 THEN 'critical'
    WHEN i <= 140 THEN 'high'
    ELSE 'normal'
  END,
  (ARRAY['Architecture', 'Structure', 'MEP', 'Landscape'])[1 + ((i - 1) % 4)],
  (ARRAY['A. Keller', 'M. Rossi', 'S. Nguyen', 'L. Weber'])[1 + ((i - 1) % 4)],
  current_date + (i % 30),
  ST_SetSRID(
    ST_MakePoint(
      8.5145 + ((i % 120) * 0.00005),
      47.3825 + ((i % 80) * 0.00005),
      410 + (i % 10)
    ),
    4326
  )
FROM generate_series(1, 240) AS i
ON CONFLICT (project_id, external_id) DO NOTHING;

INSERT INTO clashes (project_id, external_id, title, status, priority, location)
VALUES
  ('018f0000-0000-7000-8000-000000000001', 'CLASH-001', 'Structure / MEP — level 2', 'open', 'high', ST_SetSRID(ST_MakePoint(8.5162, 47.3841, 412.4), 4326)),
  ('018f0000-0000-7000-8000-000000000001', 'CLASH-002', 'Facade / Structure — north core', 'open', 'critical', ST_SetSRID(ST_MakePoint(8.5171, 47.3850, 413.1), 4326)),
  ('018f0000-0000-7000-8000-000000000001', 'CLASH-003', 'Landscape / Utilities — east yard', 'resolved', 'normal', ST_SetSRID(ST_MakePoint(8.5184, 47.3832, 409.7), 4326))
ON CONFLICT (project_id, external_id) DO NOTHING;

INSERT INTO sheets (project_id, external_id, number, title, discipline)
SELECT
  '018f0000-0000-7000-8000-000000000001',
  'SHEET-' || lpad(i::text, 2, '0'),
  (ARRAY['A', 'S', 'M', 'L'])[1 + ((i - 1) % 4)] || '-' || (100 + i)::text,
  (ARRAY['Ground floor', 'Level 1', 'Level 2', 'Roof', 'Site plan', 'Section'])[1 + ((i - 1) % 6)]
    || ' — ' || i,
  (ARRAY['Architecture', 'Structure', 'MEP', 'Landscape'])[1 + ((i - 1) % 4)]
FROM generate_series(1, 12) AS i
ON CONFLICT (project_id, external_id) DO NOTHING;

INSERT INTO datasets (id, project_id, name, kind, format, source_kind, crs, bbox, licence, attribution)
VALUES
  ('018f0000-0000-7000-8000-000000000101', '018f0000-0000-7000-8000-000000000001', 'Flood hazard zone', 'vector', 'GeoJSON', 'upload', 'EPSG:4326', ST_MakeEnvelope(8.5135, 47.3815, 8.5205, 47.3875, 4326), 'CC-BY-4.0', 'Kanton Zürich — syntheticized for demo'),
  ('018f0000-0000-7000-8000-000000000102', '018f0000-0000-7000-8000-000000000001', 'Hillshade', 'raster', 'COG', 'upload', 'EPSG:2056', ST_MakeEnvelope(8.5130, 47.3810, 8.5210, 47.3880, 4326), 'Open government', 'Swisstopo-derived — syntheticized for demo'),
  ('018f0000-0000-7000-8000-000000000103', '018f0000-0000-7000-8000-000000000001', 'Utilities', 'vector', 'GeoJSON', 'upload', 'EPSG:2056', ST_MakeEnvelope(8.5140, 47.3820, 8.5200, 47.3870, 4326), 'Synthetic', 'Fictional demo data')
ON CONFLICT (id) DO NOTHING;

INSERT INTO layers (project_id, dataset_id, name, type, visible, z_index, licence, attribution)
SELECT
  '018f0000-0000-7000-8000-000000000001',
  d.id,
  d.name,
  CASE WHEN d.kind = 'raster' THEN 'cog' ELSE 'geojson' END,
  true,
  CASE WHEN d.name = 'Flood hazard zone' THEN 20 WHEN d.name = 'Hillshade' THEN 10 ELSE 30 END,
  d.licence,
  d.attribution
FROM datasets d
WHERE d.project_id = '018f0000-0000-7000-8000-000000000001'
  AND NOT EXISTS (
    SELECT 1 FROM layers l
    WHERE l.project_id = d.project_id AND l.dataset_id = d.id
  );

INSERT INTO layers (project_id, name, type, visible, z_index, source_config)
SELECT
  '018f0000-0000-7000-8000-000000000001',
  'Issues',
  'mvt',
  true,
  40,
  '{"source":"issues"}'::jsonb
WHERE NOT EXISTS (
  SELECT 1 FROM layers WHERE project_id = '018f0000-0000-7000-8000-000000000001' AND name = 'Issues'
);

INSERT INTO jobs (project_id, type, status, progress, payload)
SELECT
  '018f0000-0000-7000-8000-000000000001',
  'connector.sync',
  'succeeded',
  100,
  '{"provider":"mock-revizto","records":255}'::jsonb
WHERE NOT EXISTS (
  SELECT 1 FROM jobs WHERE project_id = '018f0000-0000-7000-8000-000000000001' AND type = 'connector.sync'
);

INSERT INTO audit_events (project_id, action, resource_type, resource_id, metadata)
VALUES (
  '018f0000-0000-7000-8000-000000000001',
  'project.seeded',
  'project',
  '018f0000-0000-7000-8000-000000000001',
  '{"source":"fixtures/demo","synthetic":true}'::jsonb
);

INSERT INTO project_localizations (
  project_id, method, source_crs, target_crs, transform, accuracy_m, confidence, verified, notes
)
VALUES (
  '018f0000-0000-7000-8000-000000000001',
  'control_points',
  'LOCAL:PROJECT',
  'EPSG:2056',
  '{"type":"helmert2d","tx":2683120.41,"ty":1247310.87,"rotationDeg":12.4,"scale":1.000021,"rms":0.18}'::jsonb,
  0.18,
  'verified',
  true,
  'Synthetic control points; demo only'
)
ON CONFLICT (project_id) DO NOTHING;

INSERT INTO project_features (project_id, layer_key, properties, geometry)
SELECT
  '018f0000-0000-7000-8000-000000000001',
  'site',
  '{"name":"Hafenquartier site boundary","kind":"site"}'::jsonb,
  ST_GeomFromText('POLYGON((8.5144 47.3823, 8.5198 47.3823, 8.5198 47.3869, 8.5144 47.3869, 8.5144 47.3823))', 4326)
WHERE NOT EXISTS (
  SELECT 1 FROM project_features WHERE project_id = '018f0000-0000-7000-8000-000000000001' AND layer_key = 'site'
);

INSERT INTO project_features (project_id, layer_key, properties, geometry)
SELECT
  '018f0000-0000-7000-8000-000000000001',
  'flood',
  '{"name":"Flood hazard zone","kind":"flood","source":"synthetic"}'::jsonb,
  ST_GeomFromText('POLYGON((8.5144 47.3823, 8.5182 47.3823, 8.5182 47.3854, 8.5144 47.3854, 8.5144 47.3823))', 4326)
WHERE NOT EXISTS (
  SELECT 1 FROM project_features WHERE project_id = '018f0000-0000-7000-8000-000000000001' AND layer_key = 'flood'
);

INSERT INTO project_features (project_id, layer_key, properties, geometry)
SELECT
  '018f0000-0000-7000-8000-000000000001',
  'buildings',
  jsonb_build_object('name', name, 'kind', 'building', 'height', height, 'floors', floors),
  ST_GeomFromText(wkt, 4326)
FROM (
  VALUES
    ('Building A - offices', 24, 6, 'POLYGON((8.5150 47.3830, 8.5159 47.3830, 8.5159 47.3838, 8.5150 47.3838, 8.5150 47.3830))'),
    ('Building B - offices', 21, 5, 'POLYGON((8.5162 47.3830, 8.5170 47.3830, 8.5170 47.3839, 8.5162 47.3839, 8.5162 47.3830))'),
    ('Building C - residential', 27, 8, 'POLYGON((8.5150 47.3841, 8.5160 47.3841, 8.5160 47.3849, 8.5150 47.3849, 8.5150 47.3841))'),
    ('Building D - hotel', 30, 9, 'POLYGON((8.5163 47.3841, 8.5173 47.3841, 8.5173 47.3850, 8.5163 47.3850, 8.5163 47.3841))'),
    ('Building E - retail podium', 12, 3, 'POLYGON((8.5176 47.3832, 8.5190 47.3832, 8.5190 47.3840, 8.5176 47.3840, 8.5176 47.3832))'),
    ('Building F - energy centre', 9, 2, 'POLYGON((8.5177 47.3843, 8.5187 47.3843, 8.5187 47.3849, 8.5177 47.3849, 8.5177 47.3843))')
) AS buildings(name, height, floors, wkt)
WHERE NOT EXISTS (
  SELECT 1 FROM project_features WHERE project_id = '018f0000-0000-7000-8000-000000000001' AND layer_key = 'buildings'
);

INSERT INTO layers (project_id, name, type, visible, z_index, layer_key, source_config)
SELECT
  '018f0000-0000-7000-8000-000000000001',
  'Site boundary',
  'geojson',
  true,
  5,
  'site',
  '{"source":"project_features"}'::jsonb
WHERE NOT EXISTS (
  SELECT 1 FROM layers WHERE project_id = '018f0000-0000-7000-8000-000000000001' AND layer_key = 'site'
);

INSERT INTO layers (project_id, name, type, visible, z_index, layer_key, source_config)
SELECT
  '018f0000-0000-7000-8000-000000000001',
  'Buildings',
  'geojson',
  true,
  15,
  'buildings',
  '{"source":"project_features","extrusion":true}'::jsonb
WHERE NOT EXISTS (
  SELECT 1 FROM layers WHERE project_id = '018f0000-0000-7000-8000-000000000001' AND layer_key = 'buildings'
);

UPDATE layers SET layer_key = 'site' WHERE project_id = '018f0000-0000-7000-8000-000000000001' AND name = 'Site boundary';
UPDATE layers SET layer_key = 'flood' WHERE project_id = '018f0000-0000-7000-8000-000000000001' AND name = 'Flood hazard zone';
UPDATE layers SET layer_key = 'issues' WHERE project_id = '018f0000-0000-7000-8000-000000000001' AND name = 'Issues';
