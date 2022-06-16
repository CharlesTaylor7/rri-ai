import { knex } from 'knex'
import packageJson from '@@/package.json'

export default knex({
  client: 'postgres',
  connection: {
    user: 'postgres',
    database: packageJson.name.replaceAll('-', '_'),
  },
})
