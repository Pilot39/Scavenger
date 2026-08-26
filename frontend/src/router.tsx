import type { ReactNode } from 'react'
import { createBrowserRouter, Navigate, Outlet } from 'react-router-dom'
import { useAuth } from '@/context/AuthContext'
import { AppShell } from '@/components/layout/AppShell'
import { ErrorBoundary } from '@/components/ErrorBoundary'
import { LandingPage } from '@/pages/LandingPage'
import { HomePage } from '@/pages/HomePage'
import { LoginPage } from '@/pages/LoginPage'
import { NotFoundPage } from '@/pages/NotFoundPage'
import { RecyclerDashboard } from '@/pages/RecyclerDashboard'
import { IncentivesPage } from '@/pages/IncentivesPage'
import { WasteListPage } from '@/pages/WasteListPage'
import { ManufacturerDashboardPage } from '@/pages/ManufacturerDashboardPage'
import { CollectorDashboardPage } from '@/pages/CollectorDashboardPage'

// Each route gets its own boundary instance so a crash on one page doesn't
// take down the shared AppShell/nav, and navigating away naturally resets it.
function routeBoundary(element: ReactNode) {
  return <ErrorBoundary>{element}</ErrorBoundary>
}

function ProtectedLayout() {
  const { isAuthenticated, isLoading } = useAuth()
  if (isLoading) return null
  return isAuthenticated ? (
    <AppShell>
      <Outlet />
    </AppShell>
  ) : (
    <Navigate to="/login" replace />
  )
}

export const router = createBrowserRouter([
  { path: '/', element: <Navigate to="/login" replace /> },
  { path: '/login', element: routeBoundary(<LoginPage />) },
  {
    // Public landing page
    path: '/',
    element: routeBoundary(<LandingPage />),
  },
  {
    // Protected routes share AppShell and require authentication
    element: <ProtectedLayout />,
    children: [
      { path: 'dashboard', element: routeBoundary(<HomePage />) },
      { path: 'submit', element: <div>Submit Waste</div> },
      { path: 'collect', element: <div>Collect</div> },
      { path: 'incentives', element: routeBoundary(<IncentivesPage />) },
      { path: 'collect', element: routeBoundary(<CollectorDashboardPage />) },
      { path: 'incentives', element: <div>Incentives</div> },
      { path: 'transfer', element: <div>Transfer</div> },
      { path: 'history', element: <div>History</div> },
      { path: 'dashboard/recycler', element: routeBoundary(<RecyclerDashboard />) },
      { path: 'wastes', element: routeBoundary(<WasteListPage />) },
      { path: 'manufacturer', element: routeBoundary(<ManufacturerDashboardPage />) },
    ],
  },
  { path: '*', element: routeBoundary(<NotFoundPage />) },
])
