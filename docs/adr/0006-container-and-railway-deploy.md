# 0006. Multi-stage Docker image, deployed on Railway

- Status: Proposed
- Date: 2026-11-10
- Deciders: Serban
- Related requirements: NFR-3, NFR-4, FR-7

## Context
The service must run as a single container configured by environment variables and deploy on Railway, which injects
`PORT` and can health-check an HTTP path.

## Options considered
### Multi-stage Dockerfile: `rust:1-slim` builder, `gcr.io/distroless/cc-debian12` runtime
- Small image (tens of MB), no shell or package manager in production (smaller attack surface), runs as non-root.
- Dependency layer cached separately from source for faster rebuilds.
- Con: no shell for debugging inside the container.
### Same, with `debian:bookworm-slim` runtime
- Has a shell for debugging. Con: bigger image, more CVE surface.
### No Dockerfile, Railway Nixpacks/Railpack builder
- Zero config. Con: not portable to other hosts; the NFR asks for a container image we control.

## Decision
**Multi-stage Dockerfile with a distroless runtime**, non-root user. **`railway.json`** selects the Dockerfile builder
and sets `healthcheckPath: /health`. The app binds `0.0.0.0:$PORT` (default 8080).

## Consequences
- Same image runs locally (`docker run -p 8080:8080`) and on Railway.
- Debugging relies on logs (structured logs to stdout), not a shell.
- Deploying to Railway is an owner action and needs the owner's go-ahead.
