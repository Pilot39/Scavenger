import { useNavigate } from 'react-router-dom'
import { useAppTitle } from '@/hooks/useAppTitle'
import { useCollectorDashboard } from '@/hooks/useCollectorDashboard'
import { useWallet } from '@/context/WalletContext'
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/Card'
import { Badge } from '@/components/ui/Badge'
import { Button } from '@/components/ui/Button'
import { EmptyState } from '@/components/ui/EmptyState'
import { WasteType, Material } from '@/api/types'
import { formatTokenAmount, wasteTypeLabel, formatDate, formatAddress } from '@/lib/helpers'
import { Coins, ArrowDownToLine, Package, BarChart3 } from 'lucide-react'

const ALL_WASTE_TYPES = [
  WasteType.Paper,
  WasteType.PetPlastic,
  WasteType.Plastic,
  WasteType.Metal,
  WasteType.Glass
]

function WasteRow({
  material,
  onTransfer
}: {
  material: Material
  onTransfer: (id: number) => void
}) {
  return (
    <div className="flex flex-col gap-3 rounded-md border px-4 py-3 text-sm sm:flex-row sm:items-center sm:justify-between">
      <div className="flex flex-wrap items-center gap-2 sm:gap-3">
        <Badge variant="secondary">{wasteTypeLabel(material.waste_type)}</Badge>
        <span className="text-muted-foreground">ID #{material.id}</span>
        <span>{material.weight.toLocaleString()} g</span>
      </div>
      <div className="flex items-center justify-between gap-3 sm:justify-start">
        <span className="text-xs text-muted-foreground">{formatDate(material.submitted_at)}</span>
        <Button size="sm" variant="outline" onClick={() => onTransfer(material.id)}>
          Transfer
        </Button>
      </div>
    </div>
  )
}

export function CollectorDashboardPage() {
  useAppTitle('Collector Dashboard')
  const { address } = useWallet()
  const navigate = useNavigate()
  const {
    tokenBalance,
    pendingTransfers,
    collectedWastes,
    stats,
    statsByWasteType,
    isLoading,
    error,
    refetch
  } = useCollectorDashboard()

  const handleTransfer = (wasteId: number) => {
    navigate(`/wastes?transfer=${wasteId}`)
  }

  if (!address) {
    return (
      <div className="flex h-64 items-center justify-center text-muted-foreground">
        Connect your wallet to view the dashboard.
      </div>
    )
  }

  if (isLoading) {
    return (
      <div className="space-y-6 overflow-x-hidden">
        <div className="h-8 w-48 animate-pulse rounded bg-muted" />
        <div className="grid grid-cols-1 gap-4 sm:grid-cols-2 lg:grid-cols-4">
          {Array.from({ length: 4 }).map((_, i) => (
            <Card key={i}>
              <CardHeader className="pb-2">
                <div className="h-4 w-28 animate-pulse rounded bg-muted" />
              </CardHeader>
              <CardContent>
                <div className="h-8 w-20 animate-pulse rounded bg-muted" />
              </CardContent>
            </Card>
          ))}
        </div>
        <Card>
          <CardHeader>
            <div className="h-5 w-48 animate-pulse rounded bg-muted" />
          </CardHeader>
          <CardContent className="space-y-2">
            {Array.from({ length: 3 }).map((_, i) => (
              <div key={i} className="h-16 w-full animate-pulse rounded-md bg-muted" />
            ))}
          </CardContent>
        </Card>
      </div>
    )
  }

  if (error) {
    return (
      <div className="flex h-64 flex-col items-center justify-center gap-3 text-destructive">
        <p>{error}</p>
        <Button variant="outline" size="sm" onClick={refetch}>
          Retry
        </Button>
      </div>
    )
  }

  return (
    <div className="space-y-6 overflow-x-hidden px-4 py-6 sm:px-0 sm:py-0">
      <div className="flex flex-col gap-3 sm:flex-row sm:items-center sm:justify-between">
        <div>
          <h1 className="text-xl font-bold sm:text-2xl">Collector Dashboard</h1>
          <p className="text-sm text-muted-foreground">{formatAddress(address)}</p>
        </div>
      </div>

      {/* Stats row */}
      <div className="grid grid-cols-1 gap-4 sm:grid-cols-2 lg:grid-cols-4">
        <Card>
          <CardHeader className="pb-2">
            <CardTitle className="flex items-center gap-2 text-sm font-medium text-muted-foreground">
              <Coins className="h-4 w-4" /> Token Balance
            </CardTitle>
          </CardHeader>
          <CardContent>
            <p className="text-2xl font-bold">{formatTokenAmount(tokenBalance)}</p>
          </CardContent>
        </Card>

        <Card>
          <CardHeader className="pb-2">
            <CardTitle className="flex items-center gap-2 text-sm font-medium text-muted-foreground">
              <ArrowDownToLine className="h-4 w-4" /> Pending Transfers
            </CardTitle>
          </CardHeader>
          <CardContent>
            <p className="text-2xl font-bold">{pendingTransfers.length}</p>
          </CardContent>
        </Card>

        <Card>
          <CardHeader className="pb-2">
            <CardTitle className="flex items-center gap-2 text-sm font-medium text-muted-foreground">
              <Package className="h-4 w-4" /> Collected Wastes
            </CardTitle>
          </CardHeader>
          <CardContent>
            <p className="text-2xl font-bold">{collectedWastes.length}</p>
          </CardContent>
        </Card>

        <Card>
          <CardHeader className="pb-2">
            <CardTitle className="flex items-center gap-2 text-sm font-medium text-muted-foreground">
              <BarChart3 className="h-4 w-4" /> Total Transfers
            </CardTitle>
          </CardHeader>
          <CardContent>
            <p className="text-2xl font-bold">{stats?.transfers_count ?? 0}</p>
          </CardContent>
        </Card>
      </div>

      {/* Pending incoming transfers */}
      <Card>
        <CardHeader>
          <CardTitle className="text-base">Pending Incoming Transfers</CardTitle>
        </CardHeader>
        <CardContent className="space-y-2">
          {pendingTransfers.length === 0 ? (
            <EmptyState
              icon={ArrowDownToLine}
              title="No pending transfers"
              description="Transfers heading your way will appear here"
            />
          ) : (
            pendingTransfers.map((m) => (
              <WasteRow key={m.id} material={m} onTransfer={handleTransfer} />
            ))
          )}
        </CardContent>
      </Card>

      {/* Collected wastes with transfer actions */}
      <Card>
        <CardHeader>
          <CardTitle className="text-base">Collected Wastes</CardTitle>
        </CardHeader>
        <CardContent className="space-y-2">
          {collectedWastes.length === 0 ? (
            <EmptyState
              icon={Package}
              title="No collected wastes"
              description="Collected waste items will appear here"
            />
          ) : (
            collectedWastes.map((m) => (
              <WasteRow key={m.id} material={m} onTransfer={handleTransfer} />
            ))
          )}
        </CardContent>
      </Card>

      {/* Collection statistics by waste type */}
      <Card>
        <CardHeader>
          <CardTitle className="text-base">Collection Statistics by Waste Type</CardTitle>
        </CardHeader>
        <CardContent>
          <div className="grid grid-cols-2 gap-3 sm:grid-cols-3 lg:grid-cols-5">
            {ALL_WASTE_TYPES.map((type) => (
              <div
                key={type}
                className="flex flex-col items-center rounded-md border p-3 text-center"
              >
                <span className="text-xs text-muted-foreground">{wasteTypeLabel(type)}</span>
                <span className="mt-1 text-xl font-bold">{statsByWasteType[type] ?? 0}</span>
              </div>
            ))}
          </div>
        </CardContent>
      </Card>
    </div>
  )
}
