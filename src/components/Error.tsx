import { useEffect } from 'react'
import NextError from 'next/error'
import { useRouter } from 'next/router'

export interface ErrorProps {
  statusCode: string
  title?: string
}

export default function Error(props: ErrorProps) {
  const router = useRouter()

  useEffect(() => {
    // redirect to home page after 1000ms second
    setTimeout(() => router.push('/'), 1000)
  }, [])

  return (
    <NextError {...(props as any)} />
  )
}
