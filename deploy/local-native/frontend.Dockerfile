# Run `npm run build` in frontend/ first; use frontend/ as the build context.
FROM ghcr.io/krishna-swaroop/kicad-prism-frontend@sha256:a9112e171b40b1840bdb32696f16013ae36d282cf816dd4d9e9311e18e93b48c
ARG SOURCE_REVISION
LABEL org.opencontainers.image.source="https://github.com/Nrpickle/KiCAD-Prism" \
      org.opencontainers.image.revision="${SOURCE_REVISION}" \
      io.kicad-prism.local-patch="missing-pin-uuid-validation"
RUN rm -rf /usr/share/nginx/html/*
COPY dist/ /usr/share/nginx/html/
