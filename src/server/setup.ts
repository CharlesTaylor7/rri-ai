import type { Knex } from 'knex'
import db from '@/server/db'

async function defineSchema(db: Knex) {
  await db.schema
    // prettier-ignore
    .raw(`DROP TABLE IF EXISTS games CASCADE`)
    // prettier-ignore
    .raw(`
      CREATE TABLE games
        ( id SERIAL PRIMARY KEY
        , created_at TIMESTAMP DEFAULT current_timestamp
        , uuid VARCHAR(36) UNIQUE
        , client_json JSON NOT NULL DEFAULT '{}'::json
        , server_json JSON NOT NULL DEFAULT '{}'::json
        )
    `)
}

async function setupDatabase() {
  await defineSchema(db)
  await db.destroy()
}

setupDatabase()
