import useSelector from '@/hooks/useSelector'
import RouteDefinitions from '@/components/RouteDefinitions'
import DiceButton from '@/components/game/DiceButton'

export default function DicePanel() {
  const { diceCodes, round } = useSelector((state) => state)
  return (
    <div className="flex flex-col items-center">
      <div className="grid grid-cols-2 gap-4">
        <DiceButton />
        <div className="flex justify-center items-center">Round {round}</div>
        {diceCodes.map((c, i) => (
          <Die key={i} code={c} />
        ))}
      </div>
    </div>
  )
}

type DieProps = {
  code: number
}
function Die(props: DieProps) {
  return (
    <svg className="shink-0 grow-0 border rounded h-[80px] w-[80px]">
      <RouteDefinitions />
      <use href={`#route-${props.code}`} strokeWidth={1} stroke="black" />
    </svg>
  )
}
