import { apiClient } from './client'
import type { AddWishItemInput, PurchaseWishItemInput, ReviewWishItemInput, WishItem } from './types'

export function fetchWishItems(): Promise<WishItem[]> {
  return apiClient.get<WishItem[]>('/wish-items')
}

export function addWishItem(input: AddWishItemInput): Promise<WishItem> {
  return apiClient.post<WishItem>('/wish-items', input)
}

export function reviewWishItem(id: string, input: ReviewWishItemInput): Promise<void> {
  return apiClient.post<void>(`/wish-items/${id}/review`, input)
}

export function purchaseWishItem(id: string, input: PurchaseWishItemInput): Promise<void> {
  return apiClient.post<void>(`/wish-items/${id}/purchase`, input)
}
