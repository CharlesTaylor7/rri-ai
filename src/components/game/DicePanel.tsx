import useSelector from 'app/hooks/useSelector'
import RouteDefinitions from '../RouteDefinitions'
import {DrawnRoute} from './Grid'
import DiceButton from '@/components/game/DiceButton'

export default function DicePanel() {
  const diceCodes = useSelector((state) => state.diceCodes)
  return (
    <div className="m-6 flex flex-wrap gap-2 ">
      <DiceButton />
      <div/>
      {diceCodes.map((c, i) => (
        <Die key={i} code={c} /> 
      ))}
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
