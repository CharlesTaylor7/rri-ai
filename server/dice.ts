export const routes = [
    { north: 'r', west: 'r' },
    { north: 'r', east: 'r', west: 'r' },
    { north: 'r', south: 'r'},
    { north: 'h', west: 'h' },
    { north: 'h', east: 'h', west: 'h' },
    { north: 'h', south: 'h'},
    { north: 'h', south: 'h', east: 'r', west: 'r'},
    { north: 'r', south: 'h', station: true},
    { north: 'r', west: 'h', station: true},
    { north: 'h', east: 'h', south: 'r', west: 'h', station: true},
    { north: 'h', east: 'r', south: 'r', west: 'r', station: true},
    { north: 'h', east: 'h', south: 'h', west: 'h'},
    { north: 'r', east: 'r', south: 'r', west: 'r'},
    { north: 'h', east: 'r', south: 'r', west: 'h', station: true},
    { north: 'h', east: 'r', south: 'h', west: 'r', station: true},
]

export const rotate = (route, i) => {
    // copy the route
    route = {...route}
    while (i-->0) {
        const west = route.west
        route.west = route.south
        route.south = route.east
        route.east = route.north
        route.north = west
    }
    return route
}


const basicDie = Array.from({ length: 6 }, (_, i) => i)
const specialDie = Array.from({ length: 6 }, (_, i) => 6 + i % 3)
export const dice = [basicDie, basicDie, basicDie, specialDie]

export const roll = (die) => die[Math.floor(Math.random() * 6)]
