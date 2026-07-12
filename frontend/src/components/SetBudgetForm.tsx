import type { FormEvent } from 'react'
import { useState } from 'react'
import { useMutation, useQueryClient } from '@tanstack/react-query'
import { setBudget } from '../api/budgets'
import { toUserFacingError } from '../utils/errors'
import './SetBudgetForm.css'

interface SetBudgetFormProps {
  year: number
  month: number
  initialAmount?: number
  onSuccess?: () => void
  onCancel?: () => void
}

export function SetBudgetForm({ year, month, initialAmount, onSuccess, onCancel }: SetBudgetFormProps) {
  const queryClient = useQueryClient()
  const [amount, setAmount] = useState(initialAmount !== undefined ? String(initialAmount) : '')
  const [formError, setFormError] = useState<string | null>(null)

  const mutation = useMutation({
    mutationFn: setBudget,
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ['budget-status', year, month] })
      onSuccess?.()
    },
  })

  const handleSubmit = (event: FormEvent<HTMLFormElement>) => {
    event.preventDefault()

    const parsedAmount = Number(amount)
    if (!Number.isInteger(parsedAmount) || parsedAmount <= 0) {
      setFormError('予算は1円以上の整数で入力してください。')
      return
    }

    setFormError(null)
    mutation.mutate({ year, month, amount: parsedAmount })
  }

  return (
    <form className="set-budget-form" onSubmit={handleSubmit}>
      <label className="set-budget-form__field">
        <span>
          {year}年{month}月の予算
        </span>
        <input
          type="number"
          value={amount}
          onChange={(e) => setAmount(e.target.value)}
          required
        />
      </label>

      <div className="set-budget-form__actions">
        <button type="submit" disabled={mutation.isPending}>
          {initialAmount !== undefined ? '更新する' : '設定する'}
        </button>
        {onCancel && (
          <button type="button" onClick={onCancel} disabled={mutation.isPending}>
            キャンセル
          </button>
        )}
      </div>

      {formError && <p className="set-budget-form__error">{formError}</p>}

      {!formError && mutation.isError && (
        <p className="set-budget-form__error">
          {toUserFacingError(mutation.error, '保存に失敗しました。もう一度お試しください。').message}
        </p>
      )}
    </form>
  )
}
