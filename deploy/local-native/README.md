# Local native pin-identity patch

This image keeps the v5.3.0-alpha backend runtime and installs the patched
`prism-kicad` generator plus its new semantic-index schema. The frontend needs
no runtime changes: the two pin identity fields and their values are unchanged.
See [amendment P1](../../docs/architecture/adr-native-design-bundle-amendment-p1.md).

Build from the repository root after running the native quality checks:

```bash
docker build -f deploy/local-native/Dockerfile \
  --build-arg SOURCE_REVISION="$(git rev-parse HEAD)" \
  -t kicad-prism-backend:pin-uuid-local .
```

Before deployment, back up the installation's `.env` and PostgreSQL database.
Set `PRISM_BACKEND_IMAGE=kicad-prism-backend:pin-uuid-local` in that installation's
`.env`, then run `docker compose up -d --wait`. All three backend services use
the same image. Keep the projects and SSH bind mounts and database volume.
For subsequent starts, `docker compose pull --policy missing` preserves an
existing local image instead of trying to pull it from a registry.

The generator changes from 0.14.1 to 0.14.2. Request the affected revision's
design data again so Prism queues it under the new generator; its previous
failed result is not reused. Verify both the project data and application
readiness before declaring deployment successful.

To roll back, restore the former `PRISM_BACKEND_IMAGE` value and recreate the
three backend services. This patch performs no database migration. Original
design repositories are unchanged.
