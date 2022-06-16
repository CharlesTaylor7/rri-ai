import type { Knex } from 'knex'
import { connect } from '@/server/db'

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
  const db = connect()
  await defineSchema(db)
  await db.destroy()
}

setupDatabase()
