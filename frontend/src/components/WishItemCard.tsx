import type { WishItem } from '../api/types'
import { formatDate } from '../utils/date'
import { STATUS_LABELS } from '../utils/wishItemStatus'
import './WishItemCard.css'

interface WishItemCardProps {
  item: WishItem
  onReview: (stillWant: boolean) => void
  isReviewing: boolean
  reviewError?: string
}

export function WishItemCard({ item, onReview, isReviewing, reviewError }: WishItemCardProps) {
  return (
    <li className="wish-item-card">
      <div className="wish-item-card__header">
        <h3 className="wish-item-card__name">{item.name}</h3>
        <span
          className={`wish-item-card__status wish-item-card__status--${item.status.toLowerCase()}`}
        >
          {STATUS_LABELS[item.status]}
        </span>
      </div>

      <dl className="wish-item-card__details">
        <div>
          <dt>価格</dt>
          <dd>￥{item.price.toLocaleString()}</dd>
        </div>
        <div>
          <dt>カテゴリ</dt>
          <dd>{item.category_name}</dd>
        </div>
        {item.memo && (
          <div>
            <dt>メモ</dt>
            <dd>{item.memo}</dd>
          </div>
        )}
        <div>
          <dt>登録日</dt>
          <dd>{formatDate(item.added_at)}</dd>
        </div>
      </dl>

      {item.status === 'Inbox' && (
        <div className="wish-item-card__actions">
          <button type="button" disabled={isReviewing} onClick={() => onReview(true)}>
            欲しい
          </button>
          <button type="button" disabled={isReviewing} onClick={() => onReview(false)}>
            やめておく
          </button>
        </div>
      )}

      {reviewError && <p className="wish-item-card__error">{reviewError}</p>}
    </li>
  )
}
