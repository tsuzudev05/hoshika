import { apiClient } from './client'
import type { BudgetStatus } from './types'

export function fetchBudgetStatus(year: number, month: number): Promise<BudgetStatus> {
  return apiClient.get<BudgetStatus>(`/budgets/status?year=${year}&month=${month}`)
}
