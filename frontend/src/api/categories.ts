import { apiClient } from './client'
import type { Category } from './types'

export function fetchCategories(): Promise<Category[]> {
  return apiClient.get<Category[]>('/categories')
}
