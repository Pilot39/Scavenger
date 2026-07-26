import * as Dialog from '@radix-ui/react-dialog';
import { X } from 'lucide-react';
import { useWallet } from '@/context/WalletContext';
import { Button } from '@/components/ui/Button';

const WALLETS = [
  {
    id: 'freighter',
    name: 'Freighter',
    icon: '🔑',
    installUrl: 'https://freighter.app',
  },
  {
    id: 'albedo',
    name: 'Albedo',
    icon: '🌐',
    installUrl: 'https://albedo.link',
  },
];

interface WalletModalProps {
  open: boolean;
  onOpenChange: (open: boolean) => void;
}

export function WalletModal({ open, onOpenChange }: WalletModalProps) {
  const { address, isConnected, isInstalled, connect, disconnect, isLoading, error } = useWallet();

  const handleConnect = async (walletId: string) => {
    if (walletId !== 'freighter') return; // only Freighter supported via API
    await connect();
  };

  return (
    <Dialog.Root open={open} onOpenChange={onOpenChange}>
      <Dialog.Portal>
        <Dialog.Overlay className="fixed inset-0 bg-black/50 z-40" />
        <Dialog.Content className="fixed left-1/2 top-1/2 z-50 w-full max-w-sm -translate-x-1/2 -translate-y-1/2 rounded-lg border bg-card p-6 shadow-lg">
          <div className="flex items-center justify-between mb-4">
            <Dialog.Title className="text-lg font-semibold">
              {isConnected ? 'Wallet Connected' : 'Connect Wallet'}
            </Dialog.Title>
            <Dialog.Close asChild>
              <button
                className="rounded-sm text-muted-foreground hover:text-foreground focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2"
                aria-label="Close wallet dialog"
              >
                <X size={18} aria-hidden="true" />
              </button>
            </Dialog.Close>
          </div>
          <Dialog.Description className="sr-only">
            {isConnected
              ? 'Your wallet is currently connected. You can disconnect it here.'
              : 'Select a wallet to connect to the Scavngr platform.'}
          </Dialog.Description>

          {isConnected ? (
            <div className="space-y-4">
              <p className="text-sm text-muted-foreground">Connected as</p>
              <p className="font-mono text-sm bg-muted rounded px-3 py-2">{address}</p>
              <Button variant="destructive" className="w-full" autoFocus onClick={() => { disconnect(); onOpenChange(false); }}>
                Disconnect
              </Button>
            </div>
          ) : (
            <div className="space-y-3">
              {error && (
                <p role="alert" aria-live="assertive" className="text-sm text-destructive">
                  {error}
                </p>
              )}
              {WALLETS.map((wallet) => {
                const supported = wallet.id === 'freighter';
                const notInstalled = supported && !isInstalled;
                return (
                  <div key={wallet.id} className="flex items-center justify-between rounded-md border px-4 py-3">
                    <span className="flex items-center gap-2 text-sm font-medium">
                      <span>{wallet.icon}</span> {wallet.name}
                    </span>
                    {notInstalled ? (
                      <a href={wallet.installUrl} target="_blank" rel="noreferrer" className="text-xs text-primary underline">
                        Install
                      </a>
                    ) : supported ? (
                      <Button size="sm" onClick={() => handleConnect(wallet.id)} disabled={isLoading}>
                        {isLoading ? 'Connecting…' : 'Connect'}
                      </Button>
                    ) : (
                      <span className="text-xs text-muted-foreground">Coming soon</span>
                    )}
                  </div>
                );
              })}
            </div>
          )}
        </Dialog.Content>
      </Dialog.Portal>
    </Dialog.Root>
  );
}
