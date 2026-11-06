# Amendment P1: schematic pin identities without saved UUIDs

The native renderer and connectivity parser use `<symbolUuid>__pin__<token>`
when a placed pin has no saved UUID. The token is the existing sanitized pin
number, using ASCII letters, digits, underscore, dot, colon and hyphen.
The semantic index already preserves these values in `pinUuids`,
`schematicPinUuid` and its selection indexes. The a2 schema incorrectly
requires every such value to be a plain KiCad UUID, causing a newly built
archive to fail its own verification.

The a3 semantic-index contract changes only those two pin fields to accept
either a plain UUID or that renderer fallback. All other UUID fields remain
strict. Pin values and selection indexes retain their existing meaning and
bytes; no identifiers are fabricated or removed from the design sources.

New bundles declare `prism.semantic_index.a3` and generator version 0.14.2.
The a2 schema, example and generated types remain registered for stored
bundles. The backend and browser consume the same fields as before.
The new generator identity prevents reuse of failed or stale generator data.

Regression coverage builds and verifies a schematic with both saved and
missing pin UUIDs, checks the sanitized fallback and its selection references,
and rejects malformed pin identifiers and non-UUID wire identifiers. Fixtures
contain no user designs. Local deployment validation uses a read-only copy of
the affected project in an isolated container before replacing the native
binary in the existing backend image.
