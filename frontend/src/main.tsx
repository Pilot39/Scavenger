import React from 'react'
import ReactDOM from 'react-dom/client'
import { QueryClient, QueryClientProvider, MutationCache, QueryCache } from '@tanstack/react-query'
import { ReactQueryDevtools } from '@tanstack/react-query-devtools'
import { Toaster, toast } from 'sonner'
import { App } from '@/App'
import { AuthProvider } from '@/context/AuthContext'
import { WalletProvider } from '@/context/WalletContext'
import { ContractProvider } from '@/context/ContractContext'
import { ThemeProvider } from '@/context/ThemeProvider'
import { ErrorBoundary } from '@/components/ErrorBoundary'
import { getErrorMessage } from '@/lib/contractErrors'
import './index.css'

// Create a client
const queryClient = new QueryClient({
  queryCache: new QueryCache({
    onError: (error) => toast.error(getErrorMessage(error))
  }),
  mutationCache: new MutationCache({
    onError: (error) => toast.error(getErrorMessage(error))
  }),
  defaultOptions: {
    queries: {
      refetchOnWindowFocus: false,
      retry: 1,
      staleTime: 5 * 60 * 1000 // 5 minutes
    }
  }
})

ReactDOM.createRoot(document.getElementById('root')!).render(
  <React.StrictMode>
    <QueryClientProvider client={queryClient}>
      <ThemeProvider attribute="class" defaultTheme="system" enableSystem>
        <ErrorBoundary>
          <WalletProvider>
            <AuthProvider>
              <ContractProvider>
                <App />
                <Toaster position="top-right" richColors closeButton />
              </ContractProvider>
            </AuthProvider>
          </WalletProvider>
        </ErrorBoundary>
      </ThemeProvider>
      {import.meta.env.DEV && <ReactQueryDevtools initialIsOpen={false} />}
    </QueryClientProvider>
  </React.StrictMode>
)
