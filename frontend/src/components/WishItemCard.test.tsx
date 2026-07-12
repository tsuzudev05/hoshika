import { render, screen } from '@testing-library/react'
import userEvent from '@testing-library/user-event'
import { describe, expect, it, vi } from 'vitest'
import type { WishItem } from '../api/types'
import { formatDate } from '../utils/date'
import { WishItemCard } from './WishItemCard'

const baseItem: WishItem = {
  id: '11111111-1111-1111-1111-111111111111',
  name: 'リーダブルコード',
  price: 2400,
  category_name: '書籍',
  status: 'Inbox',
  memo: 'チーム内で話題',
  added_at: '2026-07-05T00:00:00Z',
}

const noop = () => {}
const defaultProps = {
  onReview: noop,
  isReviewing: false,
  onPurchase: noop,
  isPurchasing: false,
}

describe('WishItemCard', () => {
  it('アイテムの詳細を表示する', () => {
    render(<WishItemCard item={baseItem} {...defaultProps} />)

    expect(screen.getByText('リーダブルコード')).toBeInTheDocument()
    expect(screen.getByText('￥2,400')).toBeInTheDocument()
    expect(screen.getByText('書籍')).toBeInTheDocument()
    expect(screen.getByText('チーム内で話題')).toBeInTheDocument()
    expect(screen.getByText(formatDate(baseItem.added_at))).toBeInTheDocument()
    expect(screen.getByText('検討中')).toBeInTheDocument()
  })

  it('メモが空のときはメモ欄を表示しない', () => {
    render(<WishItemCard item={{ ...baseItem, memo: '' }} {...defaultProps} />)

    expect(screen.queryByText('メモ')).not.toBeInTheDocument()
  })

  it('Inboxのときだけレビューボタンを表示する', () => {
    const { rerender } = render(<WishItemCard item={baseItem} {...defaultProps} />)

    expect(screen.getByRole('button', { name: '欲しい' })).toBeInTheDocument()
    expect(screen.getByRole('button', { name: 'やめておく' })).toBeInTheDocument()

    rerender(<WishItemCard item={{ ...baseItem, status: 'NextToBuy' }} {...defaultProps} />)

    expect(screen.queryByRole('button', { name: '欲しい' })).not.toBeInTheDocument()
    expect(screen.queryByRole('button', { name: 'やめておく' })).not.toBeInTheDocument()
  })

  it('ボタンを押すとonReviewが正しい引数で呼ばれる', async () => {
    const onReview = vi.fn()
    const user = userEvent.setup()
    render(<WishItemCard item={baseItem} {...defaultProps} onReview={onReview} />)

    await user.click(screen.getByRole('button', { name: '欲しい' }))
    expect(onReview).toHaveBeenCalledWith(true)

    await user.click(screen.getByRole('button', { name: 'やめておく' }))
    expect(onReview).toHaveBeenCalledWith(false)
  })

  it('isReviewingのときはボタンが無効になる', () => {
    render(<WishItemCard item={baseItem} {...defaultProps} isReviewing={true} />)

    expect(screen.getByRole('button', { name: '欲しい' })).toBeDisabled()
    expect(screen.getByRole('button', { name: 'やめておく' })).toBeDisabled()
  })

  it('reviewErrorがあるときはメッセージと詳細を表示する', () => {
    render(
      <WishItemCard
        item={baseItem}
        {...defaultProps}
        reviewError={{ message: '更新に失敗しました。', detail: 'wish item not found' }}
      />,
    )

    expect(screen.getByText('更新に失敗しました。')).toBeInTheDocument()
    expect(screen.getByText('詳細: wish item not found')).toBeInTheDocument()
  })

  it('NextToBuyのときだけ「購入済みにする」ボタンを表示する', () => {
    const { rerender } = render(<WishItemCard item={baseItem} {...defaultProps} />)

    expect(screen.queryByRole('button', { name: '購入済みにする' })).not.toBeInTheDocument()

    rerender(<WishItemCard item={{ ...baseItem, status: 'NextToBuy' }} {...defaultProps} />)

    expect(screen.getByRole('button', { name: '購入済みにする' })).toBeInTheDocument()
  })

  it('「購入済みにする」を押すと実支払額フォームが表示され、価格欄には希望価格が入る', async () => {
    const user = userEvent.setup()
    render(<WishItemCard item={{ ...baseItem, status: 'NextToBuy' }} {...defaultProps} />)

    await user.click(screen.getByRole('button', { name: '購入済みにする' }))

    expect(screen.getByRole('spinbutton')).toHaveValue(2400)
    expect(screen.getByRole('button', { name: 'キャンセル' })).toBeInTheDocument()
  })

  it('実支払額フォームを送信するとonPurchaseが入力値で呼ばれる', async () => {
    const onPurchase = vi.fn()
    const user = userEvent.setup()
    render(
      <WishItemCard item={{ ...baseItem, status: 'NextToBuy' }} {...defaultProps} onPurchase={onPurchase} />,
    )

    await user.click(screen.getByRole('button', { name: '購入済みにする' }))
    const priceInput = screen.getByRole('spinbutton')
    await user.clear(priceInput)
    await user.type(priceInput, '1980')
    await user.type(screen.getByRole('textbox'), 'セールで安かった')
    await user.click(screen.getByRole('button', { name: '購入済みにする' }))

    expect(onPurchase).toHaveBeenCalledWith(1980, 'セールで安かった')
  })

  it('キャンセルを押すとフォームが閉じる', async () => {
    const user = userEvent.setup()
    render(<WishItemCard item={{ ...baseItem, status: 'NextToBuy' }} {...defaultProps} />)

    await user.click(screen.getByRole('button', { name: '購入済みにする' }))
    expect(screen.getByRole('spinbutton')).toBeInTheDocument()

    await user.click(screen.getByRole('button', { name: 'キャンセル' }))
    expect(screen.queryByRole('spinbutton')).not.toBeInTheDocument()
  })

  it('purchaseErrorがあるときはメッセージを表示する', () => {
    render(
      <WishItemCard
        item={{ ...baseItem, status: 'NextToBuy' }}
        {...defaultProps}
        purchaseError={{ message: '購入の記録に失敗しました。' }}
      />,
    )

    expect(screen.getByText('購入の記録に失敗しました。')).toBeInTheDocument()
  })
})
