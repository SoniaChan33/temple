import { ellipsify } from '@wallet-ui/react'
import {
  useTempleAccountsQuery,
  useTempleCloseMutation,
  useTempleDecrementMutation,
  useTempleIncrementMutation,
  useTempleInitializeMutation,
  useTempleProgram,
  useTempleProgramId,
  useTempleSetMutation,
} from './temple-data-access'
import { Button } from '@/components/ui/button'
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card'
import { ExplorerLink } from '../cluster/cluster-ui'
import { TempleAccount } from '@project/anchor'
import { ReactNode } from 'react'

export function TempleProgramExplorerLink() {
  const programId = useTempleProgramId()

  return <ExplorerLink address={programId.toString()} label={ellipsify(programId.toString())} />
}

export function TempleList() {
  const templeAccountsQuery = useTempleAccountsQuery()

  if (templeAccountsQuery.isLoading) {
    return <span className="loading loading-spinner loading-lg"></span>
  }

  if (!templeAccountsQuery.data?.length) {
    return (
      <div className="text-center">
        <h2 className={'text-2xl'}>No accounts</h2>
        No accounts found. Initialize one to get started.
      </div>
    )
  }

  return (
    <div className="grid lg:grid-cols-2 gap-4">
      {templeAccountsQuery.data?.map((temple) => (
        <TempleCard key={temple.address} temple={temple} />
      ))}
    </div>
  )
}

export function TempleProgramGuard({ children }: { children: ReactNode }) {
  const programAccountQuery = useTempleProgram()

  if (programAccountQuery.isLoading) {
    return <span className="loading loading-spinner loading-lg"></span>
  }

  if (!programAccountQuery.data?.value) {
    return (
      <div className="alert alert-info flex justify-center">
        <span>Program account not found. Make sure you have deployed the program and are on the correct cluster.</span>
      </div>
    )
  }

  return children
}

function TempleCard({ temple }: { temple: TempleAccount }) {
  return (
    <Card>
      <CardHeader>
        <CardTitle>Temple: {temple.data.count}</CardTitle>
        <CardDescription>
          Account: <ExplorerLink address={temple.address} label={ellipsify(temple.address)} />
        </CardDescription>
      </CardHeader>
      <CardContent>
        <div className="flex gap-4 justify-evenly">
          <TempleButtonIncrement temple={temple} />
          <TempleButtonSet temple={temple} />
          <TempleButtonDecrement temple={temple} />
          <TempleButtonClose temple={temple} />
        </div>
      </CardContent>
    </Card>
  )
}

export function TempleButtonInitialize() {
  const mutationInitialize = useTempleInitializeMutation()

  return (
    <Button onClick={() => mutationInitialize.mutateAsync()} disabled={mutationInitialize.isPending}>
      Initialize Temple {mutationInitialize.isPending && '...'}
    </Button>
  )
}

export function TempleButtonIncrement({ temple }: { temple: TempleAccount }) {
  const incrementMutation = useTempleIncrementMutation({ temple })

  return (
    <Button variant="outline" onClick={() => incrementMutation.mutateAsync()} disabled={incrementMutation.isPending}>
      Increment
    </Button>
  )
}

export function TempleButtonSet({ temple }: { temple: TempleAccount }) {
  const setMutation = useTempleSetMutation({ temple })

  return (
    <Button
      variant="outline"
      onClick={() => {
        const value = window.prompt('Set value to:', temple.data.count.toString() ?? '0')
        if (!value || parseInt(value) === temple.data.count || isNaN(parseInt(value))) {
          return
        }
        return setMutation.mutateAsync(parseInt(value))
      }}
      disabled={setMutation.isPending}
    >
      Set
    </Button>
  )
}

export function TempleButtonDecrement({ temple }: { temple: TempleAccount }) {
  const decrementMutation = useTempleDecrementMutation({ temple })

  return (
    <Button variant="outline" onClick={() => decrementMutation.mutateAsync()} disabled={decrementMutation.isPending}>
      Decrement
    </Button>
  )
}

export function TempleButtonClose({ temple }: { temple: TempleAccount }) {
  const closeMutation = useTempleCloseMutation({ temple })

  return (
    <Button
      variant="destructive"
      onClick={() => {
        if (!window.confirm('Are you sure you want to close this account?')) {
          return
        }
        return closeMutation.mutateAsync()
      }}
      disabled={closeMutation.isPending}
    >
      Close
    </Button>
  )
}
