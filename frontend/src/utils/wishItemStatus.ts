import type { WishItemStatus } from '../api/types'

export const STATUS_LABELS: Record<WishItemStatus, string> = {
  Inbox: '検討中',
  NextToBuy: '次に買う',
  OnHold: '保留中',
  Archived: '見送り',
  Purchased: '購入済み',
}
