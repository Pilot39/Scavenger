import { useState, useMemo } from 'react'
import { useForm, Controller } from 'react-hook-form'
import { zodResolver } from '@hookform/resolvers/zod'
import {
  Newspaper, Recycle, Package, Wrench, GlassWater,
  Leaf, Cpu, LocateFixed, Search, ChevronDown, Loader2
} from 'lucide-react'
import { cn } from '@/lib/utils'
import { WasteType } from '@/api/types'
import { useImageUpload } from '@/hooks/useImageUpload'
import { ImageUpload } from '@/components/ui/ImageUpload'
import { Button } from '@/components/ui/Button'
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/Card'
import { Input } from '@/components/ui/Input'
import { wasteSubmissionSchema } from '@/lib/validation/wasteSubmission'

type WeightUnit = 'grams' | 'kilograms'

export interface WasteSubmissionFormData {
  wasteType: WasteType
  weight: number
  latitude: string
  longitude: string
  notes: string
  photoCids: string[]
}

interface WasteSubmissionFormProps {
  onSubmit: (data: WasteSubmissionFormData) => Promise<void>
  onCancel: () => void
}

const WASTE_TYPE_OPTIONS = [
  { value: WasteType.Paper, label: 'Paper', icon: Newspaper },
  { value: WasteType.PetPlastic, label: 'PET Plastic', icon: Recycle },
  { value: WasteType.Plastic, label: 'Plastic', icon: Package },
  { value: WasteType.Metal, label: 'Metal', icon: Wrench },
  { value: WasteType.Glass, label: 'Glass', icon: GlassWater },
  { value: WasteType.Organic, label: 'Organic', icon: Leaf },
  { value: WasteType.Electronic, label: 'Electronic', icon: Cpu },
] as const

export function WasteSubmissionForm({ onSubmit, onCancel }: WasteSubmissionFormProps) {
  const { images, addImages, removeImage, cids, isUploading, validationError: imageValidationError } = useImageUpload()
  const [weightUnit, setWeightUnit] = useState<WeightUnit>('grams')
  const [materialSearch, setMaterialSearch] = useState('')
  const [materialDropdownOpen, setMaterialDropdownOpen] = useState(false)
  const [locating, setLocating] = useState(false)
  const [locError, setLocError] = useState<string | null>(null)
  const [submitState, setSubmitState] = useState<'idle' | 'loading' | 'success' | 'error'>('idle')
  const [submitError, setSubmitError] = useState<string | null>(null)

  const schema = useMemo(() => wasteSubmissionSchema(weightUnit), [weightUnit])

  const {
    register,
    handleSubmit,
    control,
    setValue,
    watch,
    formState: { errors },
  } = useForm<{
    wasteType: WasteType | ''
    weight: string
    latitude: string
    longitude: string
    notes: string
  }>({
    resolver: zodResolver(schema),
    defaultValues: {
      wasteType: '',
      weight: '',
      latitude: '',
      longitude: '',
      notes: '',
    },
  })

  const weightValue = watch('weight')

  const convertedWeight = useMemo(() => {
    const num = parseFloat(weightValue)
    if (isNaN(num) || num <= 0) return null
    if (weightUnit === 'grams') return `${(num / 1000).toFixed(3)} kg`
    return `${(num * 1000).toFixed(0)} g`
  }, [weightValue, weightUnit])

  const filteredMaterials = useMemo(() => {
    if (!materialSearch) return WASTE_TYPE_OPTIONS
    const lower = materialSearch.toLowerCase()
    return WASTE_TYPE_OPTIONS.filter((opt) => opt.label.toLowerCase().includes(lower))
  }, [materialSearch])

  const selectedType = watch('wasteType')
  const selectedOption = WASTE_TYPE_OPTIONS.find((opt) => opt.value === selectedType)

  const useCurrentLocation = () => {
    if (!navigator.geolocation) {
      setLocError('Geolocation is not supported by your browser.')
      return
    }
    setLocating(true)
    setLocError(null)

    navigator.geolocation.getCurrentPosition(
      (pos) => {
        setValue('latitude', pos.coords.latitude.toFixed(6), { shouldValidate: true })
        setValue('longitude', pos.coords.longitude.toFixed(6), { shouldValidate: true })
        setLocating(false)
      },
      () => {
        setLocError('Could not get location. Enter coordinates manually.')
        setLocating(false)
      },
      { timeout: 8000 }
    )
  }

  const onFormSubmit = handleSubmit(async (data) => {
    if (cids.length === 0) return

    const weightInGrams = weightUnit === 'grams'
      ? parseFloat(data.weight)
      : parseFloat(data.weight) * 1000

    setSubmitState('loading')
    setSubmitError(null)

    try {
      await onSubmit({
        wasteType: data.wasteType as WasteType,
        weight: weightInGrams,
        latitude: data.latitude,
        longitude: data.longitude,
        notes: data.notes,
        photoCids: cids,
      })
      setSubmitState('success')
    } catch (e) {
      setSubmitError(e instanceof Error ? e.message : 'Submission failed')
      setSubmitState('error')
    }
  })

  const photoError = submitState !== 'idle' || Object.keys(errors).length > 0
    ? (cids.length === 0 && images.length === 0 ? 'At least one photo is required' : null)
    : null

  if (submitState === 'success') {
    return (
      <Card>
        <CardContent className="flex flex-col items-center gap-4 py-12">
          <div className="rounded-full bg-green-100 p-3">
            <Recycle className="h-8 w-8 text-green-600" />
          </div>
          <h2 className="text-xl font-semibold">Waste Submitted Successfully</h2>
          <p className="text-sm text-muted-foreground">Your waste submission has been recorded.</p>
          <Button variant="outline" onClick={onCancel}>Done</Button>
        </CardContent>
      </Card>
    )
  }

  return (
    <form onSubmit={onFormSubmit} noValidate>
      <Card>
        <CardHeader>
          <CardTitle>Submit Waste</CardTitle>
        </CardHeader>
        <CardContent className="space-y-6">
          {/* 1. Material Classification */}
          <section>
            <label className="block text-sm font-medium mb-2">
              Material Type <span className="text-destructive">*</span>
            </label>
            <Controller
              name="wasteType"
              control={control}
              render={({ field }) => (
                <div className="relative">
                  <button
                    type="button"
                    onClick={() => setMaterialDropdownOpen(!materialDropdownOpen)}
                    className={cn(
                      'flex h-11 w-full items-center justify-between rounded-md border border-input bg-background px-3 py-2 text-sm ring-offset-background focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2',
                      !selectedOption && 'text-muted-foreground'
                    )}
                    aria-label="Select material type"
                    aria-expanded={materialDropdownOpen}
                    aria-haspopup="listbox"
                  >
                    <span className="flex items-center gap-2">
                      {selectedOption ? (
                        <>
                          <selectedOption.icon className="h-4 w-4" />
                          {selectedOption.label}
                        </>
                      ) : (
                        'Select material type'
                      )}
                    </span>
                    <ChevronDown className="h-4 w-4 opacity-50" />
                  </button>
                  {materialDropdownOpen && (
                    <div className="absolute z-50 mt-1 w-full rounded-md border bg-popover shadow-md">
                      <div className="flex items-center border-b px-3">
                        <Search className="h-4 w-4 text-muted-foreground" />
                        <input
                          type="text"
                          placeholder="Search materials..."
                          value={materialSearch}
                          onChange={(e) => setMaterialSearch(e.target.value)}
                          className="flex-1 bg-transparent px-2 py-2 text-sm outline-none placeholder:text-muted-foreground"
                          aria-label="Search materials"
                        />
                      </div>
                      <ul role="listbox" className="max-h-48 overflow-y-auto p-1">
                        {filteredMaterials.length === 0 ? (
                          <li className="px-3 py-2 text-sm text-muted-foreground">No materials found</li>
                        ) : (
                          filteredMaterials.map((opt) => {
                            const Icon = opt.icon
                            return (
                              <li
                                key={opt.value}
                                role="option"
                                aria-selected={field.value === opt.value}
                                onClick={() => {
                                  field.onChange(opt.value)
                                  setMaterialDropdownOpen(false)
                                  setMaterialSearch('')
                                }}
                                className={cn(
                                  'flex cursor-pointer items-center gap-2 rounded-sm px-3 py-2 text-sm hover:bg-accent',
                                  field.value === opt.value && 'bg-accent'
                                )}
                              >
                                <Icon className="h-4 w-4" />
                                {opt.label}
                              </li>
                            )
                          })
                        )}
                      </ul>
                    </div>
                  )}
                </div>
              )}
            />
            {errors.wasteType && (
              <p className="mt-1 text-sm text-destructive" role="alert">{errors.wasteType.message}</p>
            )}
          </section>

          {/* 2. Photo Upload */}
          <section>
            <label className="block text-sm font-medium mb-2">
              Photos <span className="text-destructive">*</span>
            </label>
            <ImageUpload
              images={images}
              onAdd={addImages}
              onRemove={removeImage}
              validationError={imageValidationError}
            />
            {photoError && (
              <p className="mt-1 text-sm text-destructive" role="alert">{photoError}</p>
            )}
          </section>

          {/* 3. Weight & Measurement */}
          <section>
            <label htmlFor="weight-input" className="block text-sm font-medium mb-2">
              Weight <span className="text-destructive">*</span>
            </label>
            <div className="flex gap-2">
              <Input
                id="weight-input"
                type="number"
                step="any"
                min="0"
                placeholder={weightUnit === 'grams' ? 'e.g. 500' : 'e.g. 0.5'}
                {...register('weight')}
              />
              <button
                type="button"
                onClick={() => setWeightUnit(weightUnit === 'grams' ? 'kilograms' : 'grams')}
                className="shrink-0 rounded-md border border-input bg-background px-4 py-2 text-sm font-medium hover:bg-accent"
                aria-label={`Switch to ${weightUnit === 'grams' ? 'kilograms' : 'grams'}`}
              >
                {weightUnit === 'grams' ? 'g' : 'kg'}
              </button>
            </div>
            {convertedWeight && (
              <p className="mt-1 text-xs text-muted-foreground">= {convertedWeight}</p>
            )}
            {errors.weight && (
              <p className="mt-1 text-sm text-destructive" role="alert">{errors.weight.message}</p>
            )}
          </section>

          {/* 4. Location */}
          <section>
            <label className="block text-sm font-medium mb-2">
              Location <span className="text-destructive">*</span>
            </label>
            <div className="grid grid-cols-2 gap-4">
              <div>
                <Input
                  type="number"
                  step="0.000001"
                  placeholder="Latitude"
                  aria-label="Latitude"
                  {...register('latitude')}
                />
                {errors.latitude && (
                  <p className="mt-1 text-sm text-destructive" role="alert">{errors.latitude.message}</p>
                )}
              </div>
              <div>
                <Input
                  type="number"
                  step="0.000001"
                  placeholder="Longitude"
                  aria-label="Longitude"
                  {...register('longitude')}
                />
                {errors.longitude && (
                  <p className="mt-1 text-sm text-destructive" role="alert">{errors.longitude.message}</p>
                )}
              </div>
            </div>
            <button
              type="button"
              onClick={useCurrentLocation}
              disabled={locating}
              className="mt-2 flex items-center gap-2 text-sm text-primary hover:underline"
            >
              <LocateFixed className={cn('h-4 w-4', locating && 'animate-pulse')} />
              {locating ? 'Locating...' : 'Use current location'}
            </button>
            {locError && (
              <p className="mt-1 text-sm text-destructive" role="alert">{locError}</p>
            )}
          </section>

          {/* 5. Notes */}
          <section>
            <label htmlFor="notes-input" className="block text-sm font-medium mb-2">
              Notes
            </label>
            <textarea
              id="notes-input"
              rows={3}
              placeholder="Optional notes about this waste submission..."
              className="flex w-full rounded-md border border-input bg-background px-3 py-2 text-sm ring-offset-background placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:cursor-not-allowed disabled:opacity-50"
              {...register('notes')}
            />
          </section>

          {/* Submit error */}
          {submitError && (
            <p className="text-sm text-destructive" role="alert">{submitError}</p>
          )}

          {/* 6. Action buttons */}
          <div className="flex gap-3 pt-2">
            <Button type="button" variant="outline" onClick={onCancel} className="flex-1">
              Cancel
            </Button>
            <Button
              type="submit"
              className="flex-1"
              disabled={submitState === 'loading' || isUploading}
            >
              {submitState === 'loading' ? (
                <>
                  <Loader2 className="mr-2 h-4 w-4 animate-spin" />
                  Submitting...
                </>
              ) : isUploading ? (
                <>
                  <Loader2 className="mr-2 h-4 w-4 animate-spin" />
                  Uploading photos...
                </>
              ) : (
                'Submit Waste'
              )}
            </Button>
          </div>
        </CardContent>
      </Card>
    </form>
  )
}
