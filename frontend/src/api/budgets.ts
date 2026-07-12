import { apiClient } from './client'
import type { BudgetStatus, SetBudgetInput } from './types'

export function fetchBudgetStatus(year: number, month: number): Promise<BudgetStatus> {
  return apiClient.get<BudgetStatus>(`/budgets/status?year=${year}&month=${month}`)
}

export function setBudget(input: SetBudgetInput): Promise<BudgetStatus> {
  return apiClient.post<BudgetStatus>('/budgets', input)
}
