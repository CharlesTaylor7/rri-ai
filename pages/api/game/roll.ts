import type { NextApiRequest, NextApiResponse } from 'next'
import { dice, roll } from 'server/dice'

type Data = {
    routeCodes: Array<number>
}

export default function handler(
    req: NextApiRequest,
    res: NextApiResponse<Data>
) {
    res.status(200).json({
        routeCodes: dice.map(die => roll(die))
    })
}
