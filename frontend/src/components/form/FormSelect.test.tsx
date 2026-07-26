import { describe, it, expect, vi } from 'vitest'
import { render, screen, waitFor } from '@testing-library/react'
import userEvent from '@testing-library/user-event'
import { FormSelect } from './FormSelect'

describe('FormSelect', () => {
  const options = [
    { value: 'option1', label: 'Option 1' },
    { value: 'option2', label: 'Option 2' },
    { value: 'option3', label: 'Option 3' },
  ]

  it('renders label', () => {
    render(
      <FormSelect
        id="test-select"
        label="Select an option"
        options={options}
      />
    )

    expect(screen.getByText('Select an option')).toBeInTheDocument()
  })

  it('associates label with select via id', () => {
    render(
      <FormSelect
        id="test-select"
        label="Test Select"
        options={options}
      />
    )

    const label = screen.getByText('Test Select')
    expect(label).toHaveAttribute('for', 'test-select')
  })

  it('renders all options', async () => {
    const user = userEvent.setup()
    const { container } = render(
      <FormSelect
        id="test-select"
        label="Select"
        options={options}
      />
    )

    const select = container.querySelector('#test-select')
    await user.click(select!)

    await waitFor(() => {
      options.forEach(option => {
        expect(screen.getByText(option.label)).toBeInTheDocument()
      })
    })
  })

  it('displays default placeholder text', () => {
    const { container } = render(
      <FormSelect
        id="test-select"
        label="Select"
        options={options}
        placeholder="Choose one..."
      />
    )

    const select = container.querySelector('#test-select') as HTMLSelectElement
    expect(select.value).toBe('')
  })

  it('allows selecting an option', async () => {
    const user = userEvent.setup()
    const handleChange = vi.fn()
    const { container } = render(
      <FormSelect
        id="test-select"
        label="Select"
        options={options}
        onChange={handleChange}
      />
    )

    const select = container.querySelector('#test-select') as HTMLSelectElement
    await user.selectOptions(select, 'option2')

    await waitFor(() => {
      expect(select.value).toBe('option2')
    })
  })

  it('calls onChange when selection changes', async () => {
    const user = userEvent.setup()
    const handleChange = vi.fn()
    const { container } = render(
      <FormSelect
        id="test-select"
        label="Select"
        options={options}
        onChange={handleChange}
      />
    )

    const select = container.querySelector('#test-select') as HTMLSelectElement
    await user.selectOptions(select, 'option1')

    expect(handleChange).toHaveBeenCalled()
  })

  it('displays error message when provided', () => {
    render(
      <FormSelect
        id="test-select"
        label="Select"
        options={options}
        error="Selection is required"
      />
    )

    expect(screen.getByText('Selection is required')).toBeInTheDocument()
  })

  it('applies error styling when error is present', () => {
    const { container } = render(
      <FormSelect
        id="test-select"
        label="Select"
        options={options}
        error="Error"
      />
    )

    const select = container.querySelector('#test-select')
    expect(select).toHaveClass('border-destructive')
  })

  it('handles disabled state', () => {
    const { container } = render(
      <FormSelect
        id="test-select"
        label="Select"
        options={options}
        disabled={true}
      />
    )

    const select = container.querySelector('#test-select') as HTMLSelectElement
    expect(select).toBeDisabled()
  })

  it('supports value prop', () => {
    const { container } = render(
      <FormSelect
        id="test-select"
        label="Select"
        options={options}
        value="option2"
      />
    )

    const select = container.querySelector('#test-select') as HTMLSelectElement
    expect(select.value).toBe('option2')
  })

  it('supports required attribute', () => {
    const { container } = render(
      <FormSelect
        id="test-select"
        label="Select"
        options={options}
        required={true}
      />
    )

    const select = container.querySelector('#test-select') as HTMLSelectElement
    expect(select).toBeRequired()
  })

  it('renders with multiple select support', () => {
    const { container } = render(
      <FormSelect
        id="test-select"
        label="Select Multiple"
        options={options}
        multiple={true}
      />
    )

    const select = container.querySelector('#test-select') as HTMLSelectElement
    expect(select).toHaveAttribute('multiple')
  })

  it('supports multiple selections when multiple attribute is set', async () => {
    const user = userEvent.setup()
    const { container } = render(
      <FormSelect
        id="test-select"
        label="Select"
        options={options}
        multiple={true}
      />
    )

    const select = container.querySelector('#test-select') as HTMLSelectElement
    await user.selectOptions(select, ['option1', 'option3'])

    await waitFor(() => {
      expect(select.selectedOptions).toHaveLength(2)
    })
  })

  it('preserves option grouping if provided', () => {
    const groupedOptions = [
      { group: 'Group A', options: [{ value: 'a1', label: 'A1' }] },
      { group: 'Group B', options: [{ value: 'b1', label: 'B1' }] },
    ]

    const { container } = render(
      <FormSelect
        id="test-select"
        label="Select"
        options={groupedOptions}
      />
    )

    const select = container.querySelector('#test-select')
    expect(select).toBeInTheDocument()
  })

  it('handles empty options array', () => {
    render(
      <FormSelect
        id="test-select"
        label="Select"
        options={[]}
      />
    )

    expect(screen.getByText('Select')).toBeInTheDocument()
  })
})
