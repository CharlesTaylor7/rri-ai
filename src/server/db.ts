import { Knex, knex } from 'knex'
import { name } from '@@/package.json'

export function connect(): Knex {
  return knex({
    client: 'postgres',
    connection: {
      user: 'postgres',
      database: name.replaceAll('-', '_'),
    },
  })
}
