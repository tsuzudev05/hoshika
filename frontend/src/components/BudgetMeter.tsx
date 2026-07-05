import { useQuery } from '@tanstack/react-query'
import { fetchBudgetStatus } from '../api/budgets'
import { ApiError } from '../api/client'
import { toUserFacingError } from '../utils/errors'
import './BudgetMeter.css'

function getCurrentYearMonth(): { year: number; month: number } {
  const now = new Date()
  return { year: now.getFullYear(), month: now.getMonth() + 1 }
}

export function BudgetMeter() {
  const { year, month } = getCurrentYearMonth()
  const { data, isPending, isError, error, refetch } = useQuery({
    queryKey: ['budget-status', year, month],
    queryFn: () => fetchBudgetStatus(year, month),
  })

  if (isPending) {
    return (
      <div className="budget-meter__status" role="status" aria-live="polite">
        <span className="budget-meter__spinner" aria-hidden="true" />
        <span>読み込み中...</span>
      </div>
    )
  }

  if (isError) {
    if (error instanceof ApiError && error.status === 404) {
      return (
        <p className="budget-meter__empty">
          {year}年{month}月の予算はまだ設定されていません
        </p>
      )
    }
    const { message, detail } = toUserFacingError(
      error,
      '予算状況を取得できませんでした。時間をおいて再度お試しください。',
    )
    return (
      <div className="budget-meter__error" role="alert">
        <p>
          {message}
          {detail && <span className="budget-meter__error-detail">詳細: {detail}</span>}
        </p>
        <button type="button" onClick={() => refetch()}>
          再試行
        </button>
      </div>
    )
  }

  const percentUsed =
    data.amount > 0 ? Math.min(100, Math.max(0, ((data.amount - data.balance) / data.amount) * 100)) : 0

  return (
    <div className="budget-meter">
      <div className="budget-meter__header">
        <h2 className="budget-meter__title">
          {data.year}年{data.month}月の予算
        </h2>
        {data.is_exceeded && <span className="budget-meter__badge">予算超過</span>}
      </div>

      <div
        className="budget-meter__bar"
        role="progressbar"
        aria-valuenow={Math.round(percentUsed)}
        aria-valuemin={0}
        aria-valuemax={100}
      >
        <div
          className={`budget-meter__bar-fill${data.is_exceeded ? ' budget-meter__bar-fill--exceeded' : ''}`}
          style={{ width: `${percentUsed}%` }}
        />
      </div>

      <dl className="budget-meter__details">
        <div>
          <dt>予算</dt>
          <dd>￥{data.amount.toLocaleString()}</dd>
        </div>
        <div>
          <dt>残高</dt>
          <dd>￥{data.balance.toLocaleString()}</dd>
        </div>
      </dl>
    </div>
  )
}
