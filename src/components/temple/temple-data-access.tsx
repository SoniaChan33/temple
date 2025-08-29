import {
  TempleAccount,
  getCloseInstruction,
  getTempleProgramAccounts,
  getTempleProgramId,
  getDecrementInstruction,
  getIncrementInstruction,
  getInitializeInstruction,
  getSetInstruction,
} from '@project/anchor'
import { useMutation, useQuery, useQueryClient } from '@tanstack/react-query'
import { useMemo } from 'react'
import { toast } from 'sonner'
import { generateKeyPairSigner } from 'gill'
import { useWalletUi } from '@wallet-ui/react'
import { useWalletTransactionSignAndSend } from '../solana/use-wallet-transaction-sign-and-send'
import { useClusterVersion } from '@/components/cluster/use-cluster-version'
import { toastTx } from '@/components/toast-tx'
import { useWalletUiSigner } from '@/components/solana/use-wallet-ui-signer'
import { install as installEd25519 } from '@solana/webcrypto-ed25519-polyfill'

// polyfill ed25519 for browsers (to allow `generateKeyPairSigner` to work)
installEd25519()

export function useTempleProgramId() {
  const { cluster } = useWalletUi()
  return useMemo(() => getTempleProgramId(cluster.id), [cluster])
}

export function useTempleProgram() {
  const { client, cluster } = useWalletUi()
  const programId = useTempleProgramId()
  const query = useClusterVersion()

  return useQuery({
    retry: false,
    queryKey: ['get-program-account', { cluster, clusterVersion: query.data }],
    queryFn: () => client.rpc.getAccountInfo(programId).send(),
  })
}

export function useTempleInitializeMutation() {
  const { cluster } = useWalletUi()
  const queryClient = useQueryClient()
  const signer = useWalletUiSigner()
  const signAndSend = useWalletTransactionSignAndSend()

  return useMutation({
    mutationFn: async () => {
      const temple = await generateKeyPairSigner()
      return await signAndSend(getInitializeInstruction({ payer: signer, temple }), signer)
    },
    onSuccess: async (tx) => {
      toastTx(tx)
      await queryClient.invalidateQueries({ queryKey: ['temple', 'accounts', { cluster }] })
    },
    onError: () => toast.error('Failed to run program'),
  })
}

export function useTempleDecrementMutation({ temple }: { temple: TempleAccount }) {
  const invalidateAccounts = useTempleAccountsInvalidate()
  const signer = useWalletUiSigner()
  const signAndSend = useWalletTransactionSignAndSend()

  return useMutation({
    mutationFn: async () => await signAndSend(getDecrementInstruction({ temple: temple.address }), signer),
    onSuccess: async (tx) => {
      toastTx(tx)
      await invalidateAccounts()
    },
  })
}

export function useTempleIncrementMutation({ temple }: { temple: TempleAccount }) {
  const invalidateAccounts = useTempleAccountsInvalidate()
  const signAndSend = useWalletTransactionSignAndSend()
  const signer = useWalletUiSigner()

  return useMutation({
    mutationFn: async () => await signAndSend(getIncrementInstruction({ temple: temple.address }), signer),
    onSuccess: async (tx) => {
      toastTx(tx)
      await invalidateAccounts()
    },
  })
}

export function useTempleSetMutation({ temple }: { temple: TempleAccount }) {
  const invalidateAccounts = useTempleAccountsInvalidate()
  const signAndSend = useWalletTransactionSignAndSend()
  const signer = useWalletUiSigner()

  return useMutation({
    mutationFn: async (value: number) =>
      await signAndSend(
        getSetInstruction({
          temple: temple.address,
          value,
        }),
        signer,
      ),
    onSuccess: async (tx) => {
      toastTx(tx)
      await invalidateAccounts()
    },
  })
}

export function useTempleCloseMutation({ temple }: { temple: TempleAccount }) {
  const invalidateAccounts = useTempleAccountsInvalidate()
  const signAndSend = useWalletTransactionSignAndSend()
  const signer = useWalletUiSigner()

  return useMutation({
    mutationFn: async () => {
      return await signAndSend(getCloseInstruction({ payer: signer, temple: temple.address }), signer)
    },
    onSuccess: async (tx) => {
      toastTx(tx)
      await invalidateAccounts()
    },
  })
}

export function useTempleAccountsQuery() {
  const { client } = useWalletUi()

  return useQuery({
    queryKey: useTempleAccountsQueryKey(),
    queryFn: async () => await getTempleProgramAccounts(client.rpc),
  })
}

function useTempleAccountsInvalidate() {
  const queryClient = useQueryClient()
  const queryKey = useTempleAccountsQueryKey()

  return () => queryClient.invalidateQueries({ queryKey })
}

function useTempleAccountsQueryKey() {
  const { cluster } = useWalletUi()

  return ['temple', 'accounts', { cluster }]
}
