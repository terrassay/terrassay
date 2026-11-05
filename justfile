set shell := ["bash", "-uc"]

default:
    @just --list

install:
    pnpm install

dev-web:
    pnpm dev:web

dev-api:
    cargo run -p terrassay-api

test:
    cargo test --workspace
    pnpm typecheck:web

build:
    cargo build --workspace
    pnpm build:web

up:
    docker compose -f infrastructure/compose/compose.yaml up --build

down:
    docker compose -f infrastructure/compose/compose.yaml down

reset:
    docker compose -f infrastructure/compose/compose.yaml down -v
    docker compose -f infrastructure/compose/compose.yaml up --build
