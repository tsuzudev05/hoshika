// バックエンド（src/application/dto, src/presentation/handlers）のDTOと1対1で対応する型定義

export type WishItemStatus = 'Inbox' | 'NextToBuy' | 'OnHold' | 'Archived' | 'Purchased'

export interface WishItem {
  id: string
  name: string
  price: number
  category_name: string
  status: WishItemStatus
  memo: string
  added_at: string
}

export interface AddWishItemInput {
  name: string
  price: number
  category_id: string
  memo?: string | null
}

export interface ReviewWishItemInput {
  still_want: boolean
}

export interface PurchaseWishItemInput {
  actual_price: number
  memo?: string | null
}

export interface BudgetStatus {
  id: string
  year: number
  month: number
  amount: number
  balance: number
  is_exceeded: boolean
}

export interface SetBudgetInput {
  year: number
  month: number
  amount: number
}

export interface Category {
  id: string
  name: string
}
