import { useState, useEffect, useCallback, useRef, useMemo } from 'react'
import { useNavigate } from 'react-router-dom'
import { useDropzone } from 'react-dropzone'
import {
  Recycle,
  Truck,
  Factory,
  Upload,
  X,
  FileText,
  MapPin,
  ChevronLeft,
  ChevronRight,
  CheckCircle2,
  AlertCircle,
  Loader2,
  Wallet,
  User,
  ShieldCheck,
  Camera,
} from 'lucide-react'
import { Button } from '@/components/ui/Button'
import { Input } from '@/components/ui/Input'
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/Card'
import { Badge } from '@/components/ui/Badge'
import { useWallet } from '@/context/WalletContext'
import { useAuth } from '@/context/AuthContext'
import { useAppTitle } from '@/hooks/useAppTitle'
import { useRegisterParticipant } from '@/hooks/useRegisterParticipant'
import { Role } from '@/api/types'
import { cn } from '@/lib/utils'
import { formatAddress, roleLabel } from '@/lib/helpers'

// ---------------------------------------------------------------------------
// Constants
// ---------------------------------------------------------------------------

const STORAGE_KEY = 'scavngr_registration_draft'
const MAX_FILE_SIZE = 10 * 1024 * 1024 // 10 MB
const ACCEPTED_DOC_TYPES: Record<string, string[]> = {
  'application/pdf': ['.pdf'],
  'image/jpeg': ['.jpg', '.jpeg'],
  'image/png': ['.png'],
}
const ACCEPTED_SELFIE_TYPES: Record<string, string[]> = {
  'image/jpeg': ['.jpg', '.jpeg'],
  'image/png': ['.png'],
}

const STEP_LABELS = ['Role', 'Profile', 'KYC', 'Confirm'] as const
const TOTAL_STEPS = STEP_LABELS.length

const ROLE_CARDS: {
  role: Role
  icon: typeof Recycle
  title: string
  description: string
}[] = [
  {
    role: Role.Recycler,
    icon: Recycle,
    title: 'Recycler',
    description:
      'Collect and submit recyclable materials to earn rewards. You are the first link in the sustainable supply chain.',
  },
  {
    role: Role.Collector,
    icon: Truck,
    title: 'Collector',
    description:
      'Transport and aggregate waste materials from recyclers. Bridge the gap between collection points and manufacturers.',
  },
  {
    role: Role.Manufacturer,
    icon: Factory,
    title: 'Manufacturer',
    description:
      'Transform recycled materials into new products. Create incentives and drive demand for sustainable materials.',
  },
]

// ---------------------------------------------------------------------------
// Types
// ---------------------------------------------------------------------------

interface FileInfo {
  name: string
  size: number
  type: string
}

interface FormDraft {
  role: Role | null
  displayName: string
  email: string
  phone: string
  bio: string
  latitude: string
  longitude: string
  agreedToTerms: boolean
  identityDoc: FileInfo | null
  selfiePhoto: FileInfo | null
}

const emptyDraft: FormDraft = {
  role: null,
  displayName: '',
  email: '',
  phone: '',
  bio: '',
  latitude: '',
  longitude: '',
  agreedToTerms: false,
  identityDoc: null,
  selfiePhoto: null,
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

function loadDraft(): FormDraft {
  try {
    const raw = localStorage.getItem(STORAGE_KEY)
    if (!raw) return { ...emptyDraft }
    const parsed = JSON.parse(raw) as Partial<FormDraft>
    return { ...emptyDraft, ...parsed }
  } catch {
    return { ...emptyDraft }
  }
}

function saveDraft(draft: FormDraft) {
  try {
    localStorage.setItem(STORAGE_KEY, JSON.stringify(draft))
  } catch {
    // storage full — silently ignore
  }
}

function clearDraft() {
  localStorage.removeItem(STORAGE_KEY)
}

function formatFileSize(bytes: number): string {
  if (bytes < 1024) return `${bytes} B`
  if (bytes < 1024 * 1024) return `${(bytes / 1024).toFixed(1)} KB`
  return `${(bytes / (1024 * 1024)).toFixed(1)} MB`
}

function isValidEmail(value: string): boolean {
  if (!value) return true // optional
  return /^[^\s@]+@[^\s@]+\.[^\s@]+$/.test(value)
}

// ---------------------------------------------------------------------------
// Sub-components
// ---------------------------------------------------------------------------

function StepIndicator({
  current,
  total,
}: {
  current: number
  total: number
}) {
  return (
    <nav aria-label="Registration progress" className="mb-8">
      <div className="flex items-center justify-between">
        {Array.from({ length: total }, (_, i) => {
          const stepNum = i + 1
          const isActive = stepNum === current
          const isComplete = stepNum < current
          return (
            <div key={stepNum} className="flex flex-1 items-center">
              <div className="flex flex-col items-center gap-1.5">
                <div
                  className={cn(
                    'flex h-9 w-9 items-center justify-center rounded-full text-sm font-semibold transition-colors',
                    isComplete && 'bg-primary text-primary-foreground',
                    isActive && 'bg-primary text-primary-foreground ring-4 ring-primary/20',
                    !isComplete && !isActive && 'bg-muted text-muted-foreground'
                  )}
                  aria-current={isActive ? 'step' : undefined}
                >
                  {isComplete ? (
                    <CheckCircle2 className="h-5 w-5" aria-hidden="true" />
                  ) : (
                    stepNum
                  )}
                </div>
                <span
                  className={cn(
                    'hidden text-xs font-medium sm:block',
                    isActive ? 'text-primary' : 'text-muted-foreground'
                  )}
                >
                  {STEP_LABELS[i]}
                </span>
              </div>
              {i < total - 1 && (
                <div
                  className={cn(
                    'mx-2 h-0.5 flex-1 rounded-full transition-colors',
                    stepNum < current ? 'bg-primary' : 'bg-muted'
                  )}
                  aria-hidden="true"
                />
              )}
            </div>
          )
        })}
      </div>
      <p className="mt-3 text-center text-sm text-muted-foreground sm:hidden">
        Step {current} of {total} — {STEP_LABELS[current - 1]}
      </p>
    </nav>
  )
}

// ---------------------------------------------------------------------------
// Main component
// ---------------------------------------------------------------------------

export function ParticipantRegistrationPage() {
  useAppTitle('Register')

  const navigate = useNavigate()
  const { address, isConnected, connect, isLoading: walletLoading, error: walletError } = useWallet()
  const { login } = useAuth()
  const registerMutation = useRegisterParticipant()

  // ---- Form state (persisted to localStorage) ----
  const [draft, setDraft] = useState<FormDraft>(loadDraft)
  const [step, setStep] = useState(1)
  const [identityFile, setIdentityFile] = useState<File | null>(null)
  const [selfieFile, setSelfieFile] = useState<File | null>(null)
  const [geoLoading, setGeoLoading] = useState(false)
  const [geoError, setGeoError] = useState<string | null>(null)
  const [validationErrors, setValidationErrors] = useState<Record<string, string>>({})
  const [isSuccess, setIsSuccess] = useState(false)

  const stepContainerRef = useRef<HTMLDivElement>(null)

  // Persist draft on change
  useEffect(() => {
    saveDraft(draft)
  }, [draft])

  // Focus management on step change
  useEffect(() => {
    stepContainerRef.current?.focus()
  }, [step])

  // ---- Draft updater ----
  const updateDraft = useCallback(<K extends keyof FormDraft>(key: K, value: FormDraft[K]) => {
    setDraft((prev) => ({ ...prev, [key]: value }))
    setValidationErrors((prev) => {
      const next = { ...prev }
      delete next[key]
      return next
    })
  }, [])

  // ---- Geolocation ----
  const requestGeolocation = useCallback(() => {
    if (!navigator.geolocation) {
      setGeoError('Geolocation is not supported by your browser.')
      return
    }
    setGeoLoading(true)
    setGeoError(null)
    navigator.geolocation.getCurrentPosition(
      (pos) => {
        setDraft((prev) => ({
          ...prev,
          latitude: String(Math.round(pos.coords.latitude * 1e6)),
          longitude: String(Math.round(pos.coords.longitude * 1e6)),
        }))
        setGeoLoading(false)
      },
      (err) => {
        setGeoError(
          err.code === err.PERMISSION_DENIED
            ? 'Location access denied. Please enter coordinates manually.'
            : 'Unable to retrieve location. Please enter coordinates manually.'
        )
        setGeoLoading(false)
      },
      { enableHighAccuracy: true, timeout: 10000 }
    )
  }, [])

  // Auto-request geolocation when reaching step 4
  useEffect(() => {
    if (step === 4 && !draft.latitude && !draft.longitude) {
      requestGeolocation()
    }
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [step])

  // ---- Identity document dropzone ----
  const onDocDrop = useCallback(
    (accepted: File[]) => {
      const file = accepted[0]
      if (!file) return
      if (file.size > MAX_FILE_SIZE) {
        setValidationErrors((prev) => ({
          ...prev,
          identityDoc: `File exceeds ${formatFileSize(MAX_FILE_SIZE)} limit.`,
        }))
        return
      }
      setIdentityFile(file)
      updateDraft('identityDoc', { name: file.name, size: file.size, type: file.type })
    },
    [updateDraft]
  )

  const docDropzone = useDropzone({
    onDrop: onDocDrop,
    accept: ACCEPTED_DOC_TYPES,
    maxFiles: 1,
    multiple: false,
  })

  // ---- Selfie dropzone ----
  const onSelfieDrop = useCallback(
    (accepted: File[]) => {
      const file = accepted[0]
      if (!file) return
      if (file.size > MAX_FILE_SIZE) {
        setValidationErrors((prev) => ({
          ...prev,
          selfiePhoto: `File exceeds ${formatFileSize(MAX_FILE_SIZE)} limit.`,
        }))
        return
      }
      setSelfieFile(file)
      updateDraft('selfiePhoto', { name: file.name, size: file.size, type: file.type })
    },
    [updateDraft]
  )

  const selfieDropzone = useDropzone({
    onDrop: onSelfieDrop,
    accept: ACCEPTED_SELFIE_TYPES,
    maxFiles: 1,
    multiple: false,
  })

  // ---- Validation per step ----
  const validateStep = useCallback(
    (s: number): boolean => {
      const errors: Record<string, string> = {}

      if (s === 1) {
        if (!draft.role) errors.role = 'Please select a role to continue.'
      }

      if (s === 2) {
        if (!draft.displayName.trim()) errors.displayName = 'Display name is required.'
        if (draft.email && !isValidEmail(draft.email))
          errors.email = 'Please enter a valid email address.'
      }

      if (s === 3) {
        if (!draft.identityDoc) errors.identityDoc = 'Please upload an identity document.'
      }

      if (s === 4) {
        if (!draft.latitude.trim()) errors.latitude = 'Latitude is required.'
        if (!draft.longitude.trim()) errors.longitude = 'Longitude is required.'
        if (!draft.agreedToTerms)
          errors.agreedToTerms = 'You must agree to the terms and conditions.'
      }

      setValidationErrors(errors)
      return Object.keys(errors).length === 0
    },
    [draft]
  )

  // ---- Navigation ----
  const goNext = useCallback(() => {
    if (!validateStep(step)) return
    setStep((s) => Math.min(s + 1, TOTAL_STEPS))
  }, [step, validateStep])

  const goBack = useCallback(() => {
    setStep((s) => Math.max(s - 1, 1))
    setValidationErrors({})
  }, [])

  // ---- Submit ----
  const handleSubmit = useCallback(() => {
    if (!validateStep(4)) return
    if (!address || !draft.role || !draft.displayName.trim()) return

    const lat = parseInt(draft.latitude, 10) || 0
    const lon = parseInt(draft.longitude, 10) || 0

    registerMutation.mutate(
      {
        address,
        role: draft.role,
        name: draft.displayName.trim(),
        lat,
        lon,
      },
      {
        onSuccess: (participant) => {
          clearDraft()
          login({ address, role: participant.role, name: participant.name })
          setIsSuccess(true)
        },
      }
    )
  }, [address, draft, login, registerMutation, validateStep])

  // ---- Summary data for step 4 ----
  const summaryItems = useMemo(
    () => [
      { label: 'Role', value: draft.role ? roleLabel(draft.role) : '-' },
      { label: 'Display Name', value: draft.displayName || '-' },
      { label: 'Email', value: draft.email || 'Not provided' },
      { label: 'Phone', value: draft.phone || 'Not provided' },
      { label: 'Bio', value: draft.bio || 'Not provided' },
      { label: 'ID Document', value: draft.identityDoc?.name || '-' },
      { label: 'Selfie', value: draft.selfiePhoto?.name || 'Not provided' },
    ],
    [draft]
  )

  // ---- Success screen ----
  if (isSuccess) {
    return (
      <div className="flex min-h-screen items-center justify-center bg-background px-4">
        <Card className="w-full max-w-md text-center">
          <CardHeader>
            <div className="mx-auto mb-4 flex h-20 w-20 items-center justify-center rounded-full bg-green-100 dark:bg-green-900/30">
              <CheckCircle2
                className="h-10 w-10 text-green-600 dark:text-green-400"
                aria-hidden="true"
              />
            </div>
            <CardTitle className="text-2xl">Registration Complete!</CardTitle>
          </CardHeader>
          <CardContent className="space-y-4">
            <p className="text-muted-foreground">
              Welcome to Scavngr, <span className="font-semibold">{draft.displayName}</span>! Your
              account has been created as a{' '}
              <Badge variant="secondary">{draft.role ? roleLabel(draft.role) : ''}</Badge>.
            </p>
            <div className="flex flex-col gap-3 pt-2">
              <div
                className="mx-auto grid grid-cols-3 gap-1.5"
                aria-hidden="true"
              >
                {Array.from({ length: 9 }, (_, i) => (
                  <div
                    key={i}
                    className={cn(
                      'h-2 w-2 rounded-full',
                      i % 3 === 0 && 'bg-green-400',
                      i % 3 === 1 && 'bg-primary',
                      i % 3 === 2 && 'bg-yellow-400'
                    )}
                    style={{
                      animation: `pulse ${1 + i * 0.15}s ease-in-out infinite alternate`,
                    }}
                  />
                ))}
              </div>
              <Button size="lg" onClick={() => navigate('/dashboard', { replace: true })}>
                Go to Dashboard
              </Button>
            </div>
          </CardContent>
        </Card>
      </div>
    )
  }

  // ---- Wallet not connected ----
  if (!isConnected) {
    return (
      <div className="flex min-h-screen items-center justify-center bg-background px-4">
        <Card className="w-full max-w-md text-center">
          <CardHeader>
            <div className="mx-auto mb-2 flex h-14 w-14 items-center justify-center rounded-full bg-primary/10">
              <Wallet className="h-7 w-7 text-primary" aria-hidden="true" />
            </div>
            <CardTitle>Connect Your Wallet</CardTitle>
          </CardHeader>
          <CardContent className="space-y-4">
            <p className="text-sm text-muted-foreground">
              Connect your Freighter wallet to begin the registration process.
            </p>
            <Button className="w-full" size="lg" onClick={connect} disabled={walletLoading}>
              <Wallet className="mr-2 h-4 w-4" aria-hidden="true" />
              {walletLoading ? 'Connecting...' : 'Connect Wallet'}
            </Button>
            {walletError && (
              <p role="alert" aria-live="assertive" className="text-sm text-destructive">
                {walletError}
              </p>
            )}
          </CardContent>
        </Card>
      </div>
    )
  }

  // ---- Multi-step form ----
  return (
    <div className="min-h-screen bg-background px-4 py-8 sm:py-12">
      <div className="mx-auto w-full max-w-2xl">
        {/* Header */}
        <div className="mb-6 text-center">
          <div className="mx-auto mb-3 flex h-12 w-12 items-center justify-center rounded-full bg-primary/10">
            <Recycle className="h-6 w-6 text-primary" aria-hidden="true" />
          </div>
          <h1 className="text-2xl font-bold sm:text-3xl">Join Scavngr</h1>
          <p className="mt-1 text-sm text-muted-foreground">
            Connected as{' '}
            <span className="font-mono text-xs">{address ? formatAddress(address) : ''}</span>
          </p>
        </div>

        {/* Step indicator */}
        <StepIndicator current={step} total={TOTAL_STEPS} />

        {/* Step content */}
        <div
          ref={stepContainerRef}
          tabIndex={-1}
          className="outline-none"
          aria-label={`Step ${step}: ${STEP_LABELS[step - 1]}`}
        >
          {/* ============ STEP 1: Role Selection ============ */}
          {step === 1 && (
            <div className="space-y-4">
              <div className="text-center">
                <h2 className="text-lg font-semibold">Choose Your Role</h2>
                <p className="text-sm text-muted-foreground">
                  Select the role that best describes your participation in the waste management
                  ecosystem.
                </p>
              </div>

              <div className="grid gap-4 sm:grid-cols-3">
                {ROLE_CARDS.map(({ role, icon: Icon, title, description }) => {
                  const selected = draft.role === role
                  return (
                    <button
                      key={role}
                      type="button"
                      aria-pressed={selected}
                      aria-label={`Select ${title} role`}
                      onClick={() => updateDraft('role', role)}
                      className={cn(
                        'flex flex-col items-center gap-3 rounded-lg border-2 p-5 text-center transition-all hover:shadow-md focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2',
                        selected
                          ? 'border-primary bg-primary/5 shadow-md'
                          : 'border-border hover:border-primary/40'
                      )}
                    >
                      <div
                        className={cn(
                          'flex h-14 w-14 items-center justify-center rounded-full transition-colors',
                          selected ? 'bg-primary text-primary-foreground' : 'bg-muted text-muted-foreground'
                        )}
                      >
                        <Icon className="h-7 w-7" aria-hidden="true" />
                      </div>
                      <span className="text-base font-semibold">{title}</span>
                      <p className="text-xs leading-relaxed text-muted-foreground">{description}</p>
                      {selected && (
                        <Badge variant="default" className="mt-1">
                          Selected
                        </Badge>
                      )}
                    </button>
                  )
                })}
              </div>

              {validationErrors.role && (
                <p role="alert" aria-live="assertive" className="flex items-center justify-center gap-1.5 text-sm text-destructive">
                  <AlertCircle className="h-4 w-4 shrink-0" aria-hidden="true" />
                  {validationErrors.role}
                </p>
              )}
            </div>
          )}

          {/* ============ STEP 2: Profile Information ============ */}
          {step === 2 && (
            <div className="space-y-4">
              <div className="text-center">
                <h2 className="text-lg font-semibold">Profile Information</h2>
                <p className="text-sm text-muted-foreground">
                  Tell us a bit about yourself. Only the display name is required.
                </p>
              </div>

              <Card>
                <CardContent className="space-y-5 pt-6">
                  {/* Display Name */}
                  <div className="space-y-1.5">
                    <label htmlFor="reg-display-name" className="text-sm font-medium">
                      Display Name <span className="text-destructive">*</span>
                    </label>
                    <div className="relative">
                      <User className="absolute left-3 top-1/2 h-4 w-4 -translate-y-1/2 text-muted-foreground" aria-hidden="true" />
                      <Input
                        id="reg-display-name"
                        placeholder="Your display name"
                        value={draft.displayName}
                        onChange={(e) => updateDraft('displayName', e.target.value)}
                        className="pl-10"
                        autoFocus
                        aria-required="true"
                        aria-invalid={!!validationErrors.displayName}
                        aria-describedby={
                          validationErrors.displayName ? 'err-display-name' : undefined
                        }
                      />
                    </div>
                    {validationErrors.displayName && (
                      <p
                        id="err-display-name"
                        role="alert"
                        className="flex items-center gap-1.5 text-xs text-destructive"
                      >
                        <AlertCircle className="h-3.5 w-3.5 shrink-0" aria-hidden="true" />
                        {validationErrors.displayName}
                      </p>
                    )}
                  </div>

                  {/* Email */}
                  <div className="space-y-1.5">
                    <label htmlFor="reg-email" className="text-sm font-medium">
                      Email <span className="text-xs text-muted-foreground">(optional)</span>
                    </label>
                    <Input
                      id="reg-email"
                      type="email"
                      placeholder="you@example.com"
                      value={draft.email}
                      onChange={(e) => updateDraft('email', e.target.value)}
                      aria-invalid={!!validationErrors.email}
                      aria-describedby={validationErrors.email ? 'err-email' : undefined}
                    />
                    {validationErrors.email && (
                      <p
                        id="err-email"
                        role="alert"
                        className="flex items-center gap-1.5 text-xs text-destructive"
                      >
                        <AlertCircle className="h-3.5 w-3.5 shrink-0" aria-hidden="true" />
                        {validationErrors.email}
                      </p>
                    )}
                  </div>

                  {/* Phone */}
                  <div className="space-y-1.5">
                    <label htmlFor="reg-phone" className="text-sm font-medium">
                      Phone <span className="text-xs text-muted-foreground">(optional)</span>
                    </label>
                    <Input
                      id="reg-phone"
                      type="tel"
                      placeholder="+1 (555) 000-0000"
                      value={draft.phone}
                      onChange={(e) => updateDraft('phone', e.target.value)}
                    />
                  </div>

                  {/* Bio */}
                  <div className="space-y-1.5">
                    <label htmlFor="reg-bio" className="text-sm font-medium">
                      Bio{' '}
                      <span className="text-xs text-muted-foreground">
                        (optional, {200 - draft.bio.length} chars remaining)
                      </span>
                    </label>
                    <textarea
                      id="reg-bio"
                      placeholder="Tell others what you do..."
                      value={draft.bio}
                      onChange={(e) => {
                        if (e.target.value.length <= 200) updateDraft('bio', e.target.value)
                      }}
                      maxLength={200}
                      rows={3}
                      className="flex w-full rounded-md border border-input bg-background px-3 py-2 text-sm ring-offset-background placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:cursor-not-allowed disabled:opacity-50"
                    />
                  </div>
                </CardContent>
              </Card>
            </div>
          )}

          {/* ============ STEP 3: KYC Document Upload ============ */}
          {step === 3 && (
            <div className="space-y-4">
              <div className="text-center">
                <h2 className="text-lg font-semibold">Identity Verification</h2>
                <p className="text-sm text-muted-foreground">
                  Upload a government-issued ID for KYC verification.
                </p>
              </div>

              <Card>
                <CardContent className="space-y-6 pt-6">
                  {/* Identity Document */}
                  <div className="space-y-2">
                    <label className="flex items-center gap-2 text-sm font-medium">
                      <ShieldCheck className="h-4 w-4 text-primary" aria-hidden="true" />
                      Identity Document <span className="text-destructive">*</span>
                    </label>
                    <p className="text-xs text-muted-foreground">
                      Passport, driver's license, or national ID. PDF, JPEG, or PNG up to 10 MB.
                    </p>

                    {!identityFile && !draft.identityDoc ? (
                      <div
                        {...docDropzone.getRootProps()}
                        className={cn(
                          'flex cursor-pointer flex-col items-center justify-center gap-2 rounded-lg border-2 border-dashed px-4 py-8 text-center transition-colors',
                          docDropzone.isDragActive
                            ? 'border-primary bg-primary/5'
                            : 'border-border hover:border-primary/50 hover:bg-muted/40'
                        )}
                      >
                        <input
                          {...docDropzone.getInputProps()}
                          aria-label="Upload identity document"
                        />
                        <Upload className="h-8 w-8 text-muted-foreground" aria-hidden="true" />
                        <p className="text-sm font-medium">
                          {docDropzone.isDragActive
                            ? 'Drop your document here'
                            : 'Drag & drop or click to upload'}
                        </p>
                        <p className="text-xs text-muted-foreground">
                          PDF, JPEG, PNG -- max 10 MB
                        </p>
                      </div>
                    ) : (
                      <div className="flex items-center justify-between rounded-lg border bg-muted/30 p-3">
                        <div className="flex items-center gap-3">
                          <div className="flex h-10 w-10 items-center justify-center rounded-md bg-primary/10">
                            <FileText className="h-5 w-5 text-primary" aria-hidden="true" />
                          </div>
                          <div>
                            <p className="text-sm font-medium">
                              {identityFile?.name ?? draft.identityDoc?.name}
                            </p>
                            <p className="text-xs text-muted-foreground">
                              {formatFileSize(
                                identityFile?.size ?? draft.identityDoc?.size ?? 0
                              )}
                            </p>
                          </div>
                        </div>
                        <button
                          type="button"
                          aria-label="Remove identity document"
                          onClick={() => {
                            setIdentityFile(null)
                            updateDraft('identityDoc', null)
                          }}
                          className="rounded-full p-1.5 text-muted-foreground transition-colors hover:bg-destructive/10 hover:text-destructive focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring"
                        >
                          <X className="h-4 w-4" />
                        </button>
                      </div>
                    )}

                    {validationErrors.identityDoc && (
                      <p
                        role="alert"
                        aria-live="assertive"
                        className="flex items-center gap-1.5 text-xs text-destructive"
                      >
                        <AlertCircle className="h-3.5 w-3.5 shrink-0" aria-hidden="true" />
                        {validationErrors.identityDoc}
                      </p>
                    )}
                  </div>

                  {/* Selfie Photo */}
                  <div className="space-y-2">
                    <label className="flex items-center gap-2 text-sm font-medium">
                      <Camera className="h-4 w-4 text-primary" aria-hidden="true" />
                      Selfie Photo{' '}
                      <span className="text-xs text-muted-foreground">(optional)</span>
                    </label>
                    <p className="text-xs text-muted-foreground">
                      A clear photo of your face for additional verification. JPEG or PNG up to 10
                      MB.
                    </p>

                    {!selfieFile && !draft.selfiePhoto ? (
                      <div
                        {...selfieDropzone.getRootProps()}
                        className={cn(
                          'flex cursor-pointer flex-col items-center justify-center gap-2 rounded-lg border-2 border-dashed px-4 py-6 text-center transition-colors',
                          selfieDropzone.isDragActive
                            ? 'border-primary bg-primary/5'
                            : 'border-border hover:border-primary/50 hover:bg-muted/40'
                        )}
                      >
                        <input
                          {...selfieDropzone.getInputProps()}
                          aria-label="Upload selfie photo"
                        />
                        <Camera className="h-6 w-6 text-muted-foreground" aria-hidden="true" />
                        <p className="text-sm font-medium">
                          {selfieDropzone.isDragActive
                            ? 'Drop your photo here'
                            : 'Upload a selfie'}
                        </p>
                      </div>
                    ) : (
                      <div className="flex items-center justify-between rounded-lg border bg-muted/30 p-3">
                        <div className="flex items-center gap-3">
                          <div className="flex h-10 w-10 items-center justify-center rounded-md bg-primary/10">
                            <Camera className="h-5 w-5 text-primary" aria-hidden="true" />
                          </div>
                          <div>
                            <p className="text-sm font-medium">
                              {selfieFile?.name ?? draft.selfiePhoto?.name}
                            </p>
                            <p className="text-xs text-muted-foreground">
                              {formatFileSize(
                                selfieFile?.size ?? draft.selfiePhoto?.size ?? 0
                              )}
                            </p>
                          </div>
                        </div>
                        <button
                          type="button"
                          aria-label="Remove selfie photo"
                          onClick={() => {
                            setSelfieFile(null)
                            updateDraft('selfiePhoto', null)
                          }}
                          className="rounded-full p-1.5 text-muted-foreground transition-colors hover:bg-destructive/10 hover:text-destructive focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring"
                        >
                          <X className="h-4 w-4" />
                        </button>
                      </div>
                    )}

                    {validationErrors.selfiePhoto && (
                      <p
                        role="alert"
                        className="flex items-center gap-1.5 text-xs text-destructive"
                      >
                        <AlertCircle className="h-3.5 w-3.5 shrink-0" aria-hidden="true" />
                        {validationErrors.selfiePhoto}
                      </p>
                    )}
                  </div>
                </CardContent>
              </Card>
            </div>
          )}

          {/* ============ STEP 4: Location & Confirmation ============ */}
          {step === 4 && (
            <div className="space-y-4">
              <div className="text-center">
                <h2 className="text-lg font-semibold">Location & Confirmation</h2>
                <p className="text-sm text-muted-foreground">
                  Verify your location and review your information before submitting.
                </p>
              </div>

              {/* Location */}
              <Card>
                <CardHeader className="pb-3">
                  <CardTitle className="flex items-center gap-2 text-base">
                    <MapPin className="h-4 w-4 text-primary" aria-hidden="true" />
                    Location
                  </CardTitle>
                </CardHeader>
                <CardContent className="space-y-4">
                  <div className="grid grid-cols-1 gap-4 sm:grid-cols-2">
                    <div className="space-y-1.5">
                      <label htmlFor="reg-latitude" className="text-sm font-medium">
                        Latitude (microdegrees) <span className="text-destructive">*</span>
                      </label>
                      <Input
                        id="reg-latitude"
                        type="text"
                        inputMode="numeric"
                        placeholder="e.g. 37774929"
                        value={draft.latitude}
                        onChange={(e) => updateDraft('latitude', e.target.value)}
                        aria-required="true"
                        aria-invalid={!!validationErrors.latitude}
                        aria-describedby={
                          validationErrors.latitude ? 'err-latitude' : undefined
                        }
                      />
                      {validationErrors.latitude && (
                        <p
                          id="err-latitude"
                          role="alert"
                          className="flex items-center gap-1.5 text-xs text-destructive"
                        >
                          <AlertCircle className="h-3.5 w-3.5 shrink-0" aria-hidden="true" />
                          {validationErrors.latitude}
                        </p>
                      )}
                    </div>
                    <div className="space-y-1.5">
                      <label htmlFor="reg-longitude" className="text-sm font-medium">
                        Longitude (microdegrees) <span className="text-destructive">*</span>
                      </label>
                      <Input
                        id="reg-longitude"
                        type="text"
                        inputMode="numeric"
                        placeholder="e.g. -122419418"
                        value={draft.longitude}
                        onChange={(e) => updateDraft('longitude', e.target.value)}
                        aria-required="true"
                        aria-invalid={!!validationErrors.longitude}
                        aria-describedby={
                          validationErrors.longitude ? 'err-longitude' : undefined
                        }
                      />
                      {validationErrors.longitude && (
                        <p
                          id="err-longitude"
                          role="alert"
                          className="flex items-center gap-1.5 text-xs text-destructive"
                        >
                          <AlertCircle className="h-3.5 w-3.5 shrink-0" aria-hidden="true" />
                          {validationErrors.longitude}
                        </p>
                      )}
                    </div>
                  </div>

                  <Button
                    type="button"
                    variant="outline"
                    size="sm"
                    onClick={requestGeolocation}
                    disabled={geoLoading}
                  >
                    {geoLoading ? (
                      <>
                        <Loader2 className="mr-2 h-4 w-4 animate-spin" aria-hidden="true" />
                        Detecting...
                      </>
                    ) : (
                      <>
                        <MapPin className="mr-2 h-4 w-4" aria-hidden="true" />
                        Detect My Location
                      </>
                    )}
                  </Button>

                  {geoError && (
                    <p role="alert" className="flex items-center gap-1.5 text-xs text-destructive">
                      <AlertCircle className="h-3.5 w-3.5 shrink-0" aria-hidden="true" />
                      {geoError}
                    </p>
                  )}
                </CardContent>
              </Card>

              {/* Summary */}
              <Card>
                <CardHeader className="pb-3">
                  <CardTitle className="flex items-center gap-2 text-base">
                    <CheckCircle2 className="h-4 w-4 text-primary" aria-hidden="true" />
                    Registration Summary
                  </CardTitle>
                </CardHeader>
                <CardContent>
                  <dl className="divide-y">
                    {summaryItems.map(({ label, value }) => (
                      <div
                        key={label}
                        className="flex items-center justify-between py-2.5 text-sm"
                      >
                        <dt className="text-muted-foreground">{label}</dt>
                        <dd className="max-w-[60%] truncate font-medium text-right">{value}</dd>
                      </div>
                    ))}
                    <div className="flex items-center justify-between py-2.5 text-sm">
                      <dt className="text-muted-foreground">Wallet</dt>
                      <dd className="font-mono text-xs font-medium">
                        {address ? formatAddress(address) : '-'}
                      </dd>
                    </div>
                    <div className="flex items-center justify-between py-2.5 text-sm">
                      <dt className="text-muted-foreground">Location</dt>
                      <dd className="font-mono text-xs font-medium">
                        {draft.latitude && draft.longitude
                          ? `${draft.latitude}, ${draft.longitude}`
                          : '-'}
                      </dd>
                    </div>
                  </dl>
                </CardContent>
              </Card>

              {/* Terms & conditions */}
              <div className="space-y-2">
                <label className="flex items-start gap-3 cursor-pointer">
                  <input
                    type="checkbox"
                    checked={draft.agreedToTerms}
                    onChange={(e) => updateDraft('agreedToTerms', e.target.checked)}
                    className="mt-0.5 h-4 w-4 rounded border-input accent-primary focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring"
                    aria-describedby={
                      validationErrors.agreedToTerms ? 'err-terms' : undefined
                    }
                  />
                  <span className="text-sm text-muted-foreground">
                    I agree to the{' '}
                    <span className="font-medium text-foreground underline underline-offset-2">
                      Terms of Service
                    </span>{' '}
                    and{' '}
                    <span className="font-medium text-foreground underline underline-offset-2">
                      Privacy Policy
                    </span>
                    . I understand that my registration will be recorded on the Stellar blockchain.
                  </span>
                </label>
                {validationErrors.agreedToTerms && (
                  <p
                    id="err-terms"
                    role="alert"
                    aria-live="assertive"
                    className="flex items-center gap-1.5 text-xs text-destructive"
                  >
                    <AlertCircle className="h-3.5 w-3.5 shrink-0" aria-hidden="true" />
                    {validationErrors.agreedToTerms}
                  </p>
                )}
              </div>

              {/* Registration error */}
              {registerMutation.isError && (
                <p role="alert" aria-live="assertive" className="flex items-center gap-1.5 text-sm text-destructive">
                  <AlertCircle className="h-4 w-4 shrink-0" aria-hidden="true" />
                  {registerMutation.error?.message ?? 'Registration failed. Please try again.'}
                </p>
              )}
            </div>
          )}
        </div>

        {/* ---- Navigation buttons ---- */}
        <div className="mt-8 flex items-center justify-between gap-4">
          <Button
            type="button"
            variant="outline"
            onClick={goBack}
            disabled={step === 1 || registerMutation.isPending}
            aria-label="Go to previous step"
          >
            <ChevronLeft className="mr-1 h-4 w-4" aria-hidden="true" />
            Back
          </Button>

          {step < TOTAL_STEPS ? (
            <Button type="button" onClick={goNext} aria-label="Go to next step">
              Next
              <ChevronRight className="ml-1 h-4 w-4" aria-hidden="true" />
            </Button>
          ) : (
            <Button
              type="button"
              onClick={handleSubmit}
              disabled={registerMutation.isPending}
              aria-label="Submit registration"
            >
              {registerMutation.isPending ? (
                <>
                  <Loader2 className="mr-2 h-4 w-4 animate-spin" aria-hidden="true" />
                  Submitting...
                </>
              ) : (
                'Submit Registration'
              )}
            </Button>
          )}
        </div>
      </div>
    </div>
  )
}
