import type { FormEvent } from 'react'
import { useState } from 'react'
import type { WishItem } from '../api/types'
import { formatDate } from '../utils/date'
import type { UserFacingError } from '../utils/errors'
import { STATUS_LABELS } from '../utils/wishItemStatus'
import './WishItemCard.css'

interface WishItemCardProps {
  item: WishItem
  onReview: (stillWant: boolean) => void
  isReviewing: boolean
  reviewError?: UserFacingError
  onPurchase: (actualPrice: number, memo?: string) => void
  isPurchasing: boolean
  purchaseError?: UserFacingError
}

export function WishItemCard({
  item,
  onReview,
  isReviewing,
  reviewError,
  onPurchase,
  isPurchasing,
  purchaseError,
}: WishItemCardProps) {
  const [showPurchaseForm, setShowPurchaseForm] = useState(false)
  const [actualPrice, setActualPrice] = useState(String(item.price))
  const [purchaseMemo, setPurchaseMemo] = useState('')
  const [purchaseFormError, setPurchaseFormError] = useState<string | null>(null)

  const handlePurchaseSubmit = (event: FormEvent<HTMLFormElement>) => {
    event.preventDefault()

    const parsedPrice = Number(actualPrice)
    if (!Number.isInteger(parsedPrice) || parsedPrice < 0) {
      setPurchaseFormError('価格は0以上の整数で入力してください。')
      return
    }

    setPurchaseFormError(null)
    onPurchase(parsedPrice, purchaseMemo || undefined)
  }

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

      {item.status === 'NextToBuy' &&
        (showPurchaseForm ? (
          <form className="wish-item-card__purchase-form" onSubmit={handlePurchaseSubmit}>
            <label className="wish-item-card__purchase-field">
              <span>実際に支払った金額</span>
              <input
                type="number"
                min={0}
                value={actualPrice}
                onChange={(e) => setActualPrice(e.target.value)}
                required
              />
            </label>
            <label className="wish-item-card__purchase-field">
              <span>メモ（任意）</span>
              <input type="text" value={purchaseMemo} onChange={(e) => setPurchaseMemo(e.target.value)} />
            </label>
            <div className="wish-item-card__actions">
              <button type="submit" disabled={isPurchasing}>
                購入済みにする
              </button>
              <button
                type="button"
                disabled={isPurchasing}
                onClick={() => setShowPurchaseForm(false)}
              >
                キャンセル
              </button>
            </div>
            {purchaseFormError && <p className="wish-item-card__error">{purchaseFormError}</p>}
          </form>
        ) : (
          <div className="wish-item-card__actions">
            <button type="button" onClick={() => setShowPurchaseForm(true)}>
              購入済みにする
            </button>
          </div>
        ))}

      {reviewError && (
        <p className="wish-item-card__error">
          {reviewError.message}
          {reviewError.detail && (
            <span className="wish-item-card__error-detail">詳細: {reviewError.detail}</span>
          )}
        </p>
      )}

      {purchaseError && (
        <p className="wish-item-card__error">
          {purchaseError.message}
          {purchaseError.detail && (
            <span className="wish-item-card__error-detail">詳細: {purchaseError.detail}</span>
          )}
        </p>
      )}
    </li>
  )
}
