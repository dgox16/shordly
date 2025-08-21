#!/bin/sh
set -e

# Esperar a que la DB esté lista
until pg_isready -h db -p 5432; do
    echo "Waiting for database..."
    sleep 2
done

# Aplicar migraciones (solo las pendientes)
echo "Running migrations..."
sqlx migrate run || echo "No pending migrations."

# Arrancar la API
exec ./shordly_api
