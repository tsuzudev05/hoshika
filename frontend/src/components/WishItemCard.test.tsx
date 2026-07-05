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

describe('WishItemCard', () => {
  it('アイテムの詳細を表示する', () => {
    render(<WishItemCard item={baseItem} onReview={() => {}} isReviewing={false} />)

    expect(screen.getByText('リーダブルコード')).toBeInTheDocument()
    expect(screen.getByText('￥2,400')).toBeInTheDocument()
    expect(screen.getByText('書籍')).toBeInTheDocument()
    expect(screen.getByText('チーム内で話題')).toBeInTheDocument()
    expect(screen.getByText(formatDate(baseItem.added_at))).toBeInTheDocument()
    expect(screen.getByText('検討中')).toBeInTheDocument()
  })

  it('メモが空のときはメモ欄を表示しない', () => {
    render(<WishItemCard item={{ ...baseItem, memo: '' }} onReview={() => {}} isReviewing={false} />)

    expect(screen.queryByText('メモ')).not.toBeInTheDocument()
  })

  it('Inboxのときだけレビューボタンを表示する', () => {
    const { rerender } = render(<WishItemCard item={baseItem} onReview={() => {}} isReviewing={false} />)

    expect(screen.getByRole('button', { name: '欲しい' })).toBeInTheDocument()
    expect(screen.getByRole('button', { name: 'やめておく' })).toBeInTheDocument()

    rerender(
      <WishItemCard item={{ ...baseItem, status: 'NextToBuy' }} onReview={() => {}} isReviewing={false} />,
    )

    expect(screen.queryByRole('button', { name: '欲しい' })).not.toBeInTheDocument()
    expect(screen.queryByRole('button', { name: 'やめておく' })).not.toBeInTheDocument()
  })

  it('ボタンを押すとonReviewが正しい引数で呼ばれる', async () => {
    const onReview = vi.fn()
    const user = userEvent.setup()
    render(<WishItemCard item={baseItem} onReview={onReview} isReviewing={false} />)

    await user.click(screen.getByRole('button', { name: '欲しい' }))
    expect(onReview).toHaveBeenCalledWith(true)

    await user.click(screen.getByRole('button', { name: 'やめておく' }))
    expect(onReview).toHaveBeenCalledWith(false)
  })

  it('isReviewingのときはボタンが無効になる', () => {
    render(<WishItemCard item={baseItem} onReview={() => {}} isReviewing={true} />)

    expect(screen.getByRole('button', { name: '欲しい' })).toBeDisabled()
    expect(screen.getByRole('button', { name: 'やめておく' })).toBeDisabled()
  })

  it('reviewErrorがあるときはメッセージと詳細を表示する', () => {
    render(
      <WishItemCard
        item={baseItem}
        onReview={() => {}}
        isReviewing={false}
        reviewError={{ message: '更新に失敗しました。', detail: 'wish item not found' }}
      />,
    )

    expect(screen.getByText('更新に失敗しました。')).toBeInTheDocument()
    expect(screen.getByText('詳細: wish item not found')).toBeInTheDocument()
  })
})
