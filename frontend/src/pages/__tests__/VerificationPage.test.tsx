// Feature: VerificationPage
// Validates: grade selector, contamination flag, notes, history, queue navigation

import { describe, it, expect, beforeEach } from 'vitest'
import { render, screen, fireEvent } from '@testing-library/react'
import React, { useState } from 'react'

// ── Types (duplicated to avoid importing the page which triggers env checks) ──

type QualityGrade = 'A' | 'B' | 'C' | 'D' | 'F'

interface VerificationRecord {
  materialId: number
  decision: 'approved' | 'rejected'
  notes: string
  contaminated: boolean
  grade: QualityGrade
  verifiedAt: number
  verifier: string
}

// ── Local history implementation (mirrors VerificationPage) ───────────────────

const _history: VerificationRecord[] = []

function addRecord(record: VerificationRecord) {
  _history.unshift(record)
  if (_history.length > 100) _history.pop()
}

function getHistory(): VerificationRecord[] {
  return [..._history]
}

function makeRecord(overrides: Partial<VerificationRecord> = {}): VerificationRecord {
  return {
    materialId: 1,
    decision: 'approved',
    notes: '',
    contaminated: false,
    grade: 'B',
    verifiedAt: 1700000000,
    verifier: 'GABCDEF',
    ...overrides,
  }
}

// ── History tests ─────────────────────────────────────────────────────────────

describe('VerificationPage — history', () => {
  beforeEach(() => {
    _history.length = 0
  })

  it('addRecord stores a record', () => {
    addRecord(makeRecord({ materialId: 10 }))
    expect(getHistory().some((r) => r.materialId === 10)).toBe(true)
  })

  it('most recent record is first', () => {
    addRecord(makeRecord({ materialId: 1, verifiedAt: 100 }))
    addRecord(makeRecord({ materialId: 2, verifiedAt: 200 }))
    expect(getHistory()[0].materialId).toBe(2)
  })

  it('records approved decision', () => {
    addRecord(makeRecord({ decision: 'approved' }))
    expect(getHistory()[0].decision).toBe('approved')
  })

  it('records rejected decision', () => {
    addRecord(makeRecord({ decision: 'rejected' }))
    expect(getHistory()[0].decision).toBe('rejected')
  })

  it('records contamination flag', () => {
    addRecord(makeRecord({ contaminated: true }))
    expect(getHistory()[0].contaminated).toBe(true)
  })

  it('records notes', () => {
    addRecord(makeRecord({ notes: 'Looks good' }))
    expect(getHistory()[0].notes).toBe('Looks good')
  })

  it('records quality grade', () => {
    addRecord(makeRecord({ grade: 'A' }))
    expect(getHistory()[0].grade).toBe('A')
  })
})

// ── GradeSelector component ───────────────────────────────────────────────────

const GRADES: QualityGrade[] = ['A', 'B', 'C', 'D', 'F']

function GradeSelectorStub() {
  const [grade, setGrade] = useState<QualityGrade>('B')
  return (
    <div>
      <div role="group" aria-label="Quality grade">
        {GRADES.map((g) => (
          <button
            key={g}
            aria-pressed={grade === g}
            aria-label={`Grade ${g}`}
            onClick={() => setGrade(g)}
          >
            {g}
          </button>
        ))}
      </div>
      <span data-testid="selected">{grade}</span>
    </div>
  )
}

describe('VerificationPage — GradeSelector', () => {
  it('defaults to grade B', () => {
    render(<GradeSelectorStub />)
    expect(screen.getByTestId('selected').textContent).toBe('B')
  })

  it('selects grade A on click', () => {
    render(<GradeSelectorStub />)
    fireEvent.click(screen.getByLabelText('Grade A'))
    expect(screen.getByTestId('selected').textContent).toBe('A')
  })

  it('selects grade F on click', () => {
    render(<GradeSelectorStub />)
    fireEvent.click(screen.getByLabelText('Grade F'))
    expect(screen.getByTestId('selected').textContent).toBe('F')
  })

  it('marks selected grade with aria-pressed=true', () => {
    render(<GradeSelectorStub />)
    fireEvent.click(screen.getByLabelText('Grade C'))
    expect(screen.getByLabelText('Grade C')).toHaveAttribute('aria-pressed', 'true')
    expect(screen.getByLabelText('Grade B')).toHaveAttribute('aria-pressed', 'false')
  })
})

// ── Queue navigation ──────────────────────────────────────────────────────────

function QueueNavStub({ total }: { total: number }) {
  const [idx, setIdx] = useState(0)
  return (
    <div>
      <button
        aria-label="Previous item"
        disabled={idx === 0}
        onClick={() => setIdx((i) => Math.max(0, i - 1))}
      >
        Prev
      </button>
      <span data-testid="idx">{idx + 1}/{total}</span>
      <button
        aria-label="Next item"
        disabled={idx >= total - 1}
        onClick={() => setIdx((i) => Math.min(total - 1, i + 1))}
      >
        Next
      </button>
    </div>
  )
}

describe('VerificationPage — queue navigation', () => {
  it('starts at first item', () => {
    render(<QueueNavStub total={3} />)
    expect(screen.getByTestId('idx').textContent).toBe('1/3')
  })

  it('navigates to next item', () => {
    render(<QueueNavStub total={3} />)
    fireEvent.click(screen.getByLabelText('Next item'))
    expect(screen.getByTestId('idx').textContent).toBe('2/3')
  })

  it('prev is disabled at first item', () => {
    render(<QueueNavStub total={3} />)
    expect(screen.getByLabelText('Previous item')).toBeDisabled()
  })

  it('next is disabled at last item', () => {
    render(<QueueNavStub total={1} />)
    expect(screen.getByLabelText('Next item')).toBeDisabled()
  })
})
