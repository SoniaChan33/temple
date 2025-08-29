import { WalletButton } from '../solana/solana-provider'
import { TempleButtonInitialize, TempleList, TempleProgramExplorerLink, TempleProgramGuard } from './temple-ui'
import { AppHero } from '../app-hero'
import { useWalletUi } from '@wallet-ui/react'

export default function TempleFeature() {
  const { account } = useWalletUi()

  return (
    <TempleProgramGuard>
      <AppHero
        title="Temple"
        subtitle={
          account
            ? "Initialize a new temple onchain by clicking the button. Use the program's methods (increment, decrement, set, and close) to change the state of the account."
            : 'Select a wallet to run the program.'
        }
      >
        <p className="mb-6">
          <TempleProgramExplorerLink />
        </p>
        {account ? (
          <TempleButtonInitialize />
        ) : (
          <div style={{ display: 'inline-block' }}>
            <WalletButton />
          </div>
        )}
      </AppHero>
      {account ? <TempleList /> : null}
    </TempleProgramGuard>
  )
}
