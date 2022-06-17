import { Route } from '@/types'

export type Die = [number, number, number, number, number, number]

export const routes: Array<Route> = [
  { north: 'r', west: 'r' },
  { north: 'r', east: 'r', west: 'r' },
  { north: 'r', south: 'r' },
  { north: 'h', west: 'h' },
  { north: 'h', east: 'h', west: 'h' },
  { north: 'h', south: 'h' },
  { north: 'h', south: 'h', east: 'r', west: 'r' },
  { north: 'r', south: 'h', station: true },
  { north: 'r', west: 'h', station: true },
  { north: 'h', east: 'h', south: 'r', west: 'h', station: true },
  { north: 'h', east: 'r', south: 'r', west: 'r', station: true },
  { north: 'h', east: 'h', south: 'h', west: 'h' },
  { north: 'r', east: 'r', south: 'r', west: 'r' },
  { north: 'h', east: 'r', south: 'r', west: 'h', station: true },
  { north: 'h', east: 'r', south: 'h', west: 'r', station: true },
]

const basicDie: Die = [0, 1, 2, 3, 4, 5]
const specialDie: Die = [6, 7, 8, 6, 7, 8]

export const dice: Array<Die> = [basicDie, basicDie, basicDie, specialDie]
export const roll = (die: Die) => die[Math.floor(Math.random() * 6)]
