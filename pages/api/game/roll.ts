import type { NextApiRequest, NextApiResponse } from "next";
import type { RouteInfo } from "types";
import { getServerState, drawInFirstValidPosition } from "server/state";
import { dice, roll } from "server/dice";
import type { Die } from "server/dice";

type Data = {
  diceCodes: Array<number>;
  nextRoutes: Array<RouteInfo>;
};

export default function handler(
  req: NextApiRequest,
  res: NextApiResponse<Data | string>
) {
  const gameState = getServerState();
  if (gameState === undefined) {
    res.status(404).send("Game not found");
    return;
  }

  // get results
  const diceCodes = dice.map((die) => roll(die));
  const nextRoutes = diceCodes
    .map((code) => drawInFirstValidPosition(gameState, code))
    .filter((route) => route) as Array<RouteInfo>;

  res.status(200).json({
    diceCodes,
    nextRoutes,
  });
}
