import { QueryClient, QueryClientProvider } from '@tanstack/react-query'
import { render, screen, waitFor } from '@testing-library/react'
import userEvent from '@testing-library/user-event'
import { HttpResponse, http } from 'msw'
import type { ReactElement } from 'react'
import { describe, expect, it } from 'vitest'
import type { WishItem } from '../api/types'
import { server } from '../test/mocks/server'
import { WishItemList } from './WishItemList'

function renderWithQueryClient(ui: ReactElement) {
  const client = new QueryClient({ defaultOptions: { queries: { retry: false } } })
  return render(<QueryClientProvider client={client}>{ui}</QueryClientProvider>)
}

const sampleItem: WishItem = {
  id: '11111111-1111-1111-1111-111111111111',
  name: 'リーダブルコード',
  price: 2400,
  category_name: '書籍',
  status: 'Inbox',
  memo: '',
  added_at: '2026-07-05T00:00:00Z',
}

const otherCategoryItem: WishItem = {
  id: '22222222-2222-2222-2222-222222222222',
  name: 'ゲーミングマウス',
  price: 8000,
  category_name: '雑貨',
  status: 'Inbox',
  memo: '',
  added_at: '2026-07-05T00:00:00Z',
}

const sampleCategories = [
  { id: 'aaaaaaaa-aaaa-aaaa-aaaa-aaaaaaaaaaaa', name: '書籍' },
  { id: 'bbbbbbbb-bbbb-bbbb-bbbb-bbbbbbbbbbbb', name: '雑貨' },
]

function mockCategories(categories = sampleCategories) {
  server.use(http.get('/api/categories', () => HttpResponse.json(categories)))
}

describe('WishItemList', () => {
  it('読み込み中はスピナーを表示する', () => {
    mockCategories()
    server.use(
      http.get('/api/wish-items', async () => {
        await new Promise((resolve) => setTimeout(resolve, 50))
        return HttpResponse.json([])
      }),
    )

    renderWithQueryClient(<WishItemList />)

    expect(screen.getByRole('status')).toBeInTheDocument()
  })

  it('取得エラー時はメッセージと再試行ボタンを表示する', async () => {
    mockCategories()
    server.use(http.get('/api/wish-items', () => HttpResponse.json({ error: 'boom' }, { status: 500 })))

    renderWithQueryClient(<WishItemList />)

    const alert = await screen.findByRole('alert')
    expect(alert).toHaveTextContent('欲しいものリストを取得できませんでした')
    expect(screen.getByRole('button', { name: '再試行' })).toBeInTheDocument()
  })

  it('データが空のときは登録案内を表示する', async () => {
    mockCategories()
    server.use(http.get('/api/wish-items', () => HttpResponse.json([])))

    renderWithQueryClient(<WishItemList />)

    expect(await screen.findByText('欲しいものはまだ登録されていません')).toBeInTheDocument()
  })

  it('データがあるときはカードを表示する', async () => {
    mockCategories()
    server.use(http.get('/api/wish-items', () => HttpResponse.json([sampleItem])))

    renderWithQueryClient(<WishItemList />)

    expect(await screen.findByText('リーダブルコード')).toBeInTheDocument()
    expect(screen.getByText('￥2,400')).toBeInTheDocument()
  })

  it('レビュー操作が成功すると一覧が更新される', async () => {
    mockCategories()
    let reviewed = false
    server.use(
      http.get('/api/wish-items', () =>
        HttpResponse.json(reviewed ? [{ ...sampleItem, status: 'NextToBuy' }] : [sampleItem]),
      ),
      http.post('/api/wish-items/:id/review', () => {
        reviewed = true
        return HttpResponse.json({})
      }),
    )

    renderWithQueryClient(<WishItemList />)
    const user = userEvent.setup()

    await screen.findByText('リーダブルコード')
    await user.click(screen.getByRole('button', { name: '欲しい' }))

    await waitFor(() => {
      expect(screen.queryByRole('button', { name: '欲しい' })).not.toBeInTheDocument()
    })
  })

  it('カテゴリが1件もないときはフィルターを表示しない', async () => {
    mockCategories([])
    server.use(http.get('/api/wish-items', () => HttpResponse.json([sampleItem])))

    renderWithQueryClient(<WishItemList />)

    await screen.findByText('リーダブルコード')
    expect(screen.queryByRole('group', { name: 'カテゴリで絞り込み' })).not.toBeInTheDocument()
  })

  it('カテゴリを選択すると該当カテゴリのみ表示される', async () => {
    mockCategories()
    server.use(http.get('/api/wish-items', () => HttpResponse.json([sampleItem, otherCategoryItem])))

    renderWithQueryClient(<WishItemList />)
    const user = userEvent.setup()

    await screen.findByText('リーダブルコード')
    expect(screen.getByText('ゲーミングマウス')).toBeInTheDocument()

    await user.click(screen.getByRole('button', { name: '書籍' }))

    expect(screen.getByText('リーダブルコード')).toBeInTheDocument()
    expect(screen.queryByText('ゲーミングマウス')).not.toBeInTheDocument()
  })

  it('該当データがないカテゴリを選択すると絞り込み結果なしメッセージを表示する', async () => {
    mockCategories()
    server.use(http.get('/api/wish-items', () => HttpResponse.json([sampleItem])))

    renderWithQueryClient(<WishItemList />)
    const user = userEvent.setup()

    await screen.findByText('リーダブルコード')
    await user.click(screen.getByRole('button', { name: '雑貨' }))

    expect(await screen.findByText('選択したカテゴリの欲しいものはありません')).toBeInTheDocument()
  })

  it('「すべて」を選択するとフィルターが解除される', async () => {
    mockCategories()
    server.use(http.get('/api/wish-items', () => HttpResponse.json([sampleItem, otherCategoryItem])))

    renderWithQueryClient(<WishItemList />)
    const user = userEvent.setup()

    await screen.findByText('リーダブルコード')
    await user.click(screen.getByRole('button', { name: '書籍' }))
    expect(screen.queryByText('ゲーミングマウス')).not.toBeInTheDocument()

    await user.click(screen.getByRole('button', { name: 'すべて' }))
    expect(await screen.findByText('ゲーミングマウス')).toBeInTheDocument()
  })
})
