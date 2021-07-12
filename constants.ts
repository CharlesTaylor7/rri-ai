// the order of these is important
export const routeSvgs = [
    'railway-turn',
    'railway-t',
    'railway-straight',
    'highway-turn',
    'highway-t',
    'highway-straight',
    'overpass',
    'station-straight',
    'station-turn',
    'highway-t-station',
    'railway-t-station',
    'highway-four',
    'railway-four',
    'turn-station',
    'straight-station',
].map(name => `/routes/${name}.svg`)
