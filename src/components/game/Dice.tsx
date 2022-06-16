import useSelector from 'app/hooks/useSelector'

export default function Dice() {
  const diceCodes = useSelector((state) => state.diceCodes)
  return (
    <>
      {diceCodes.map((c) => (
        <div className="">{c}</div>
      ))}
    </>
  )
}
