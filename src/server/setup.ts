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
        , created_at TIMESTAMP default current_timestamp
        , uuid UUID UNIQUE
        , json JSON
        )
    `)
}

async function setupDatabase() {
  await defineSchema(db)
  await db.destroy()
}

setupDatabase()
