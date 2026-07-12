import { QueryClient, QueryClientProvider } from '@tanstack/react-query'
import { render, screen, waitFor } from '@testing-library/react'
import userEvent from '@testing-library/user-event'
import { HttpResponse, http } from 'msw'
import type { ReactElement } from 'react'
import { afterEach, beforeEach, describe, expect, it, vi } from 'vitest'
import type { BudgetStatus } from '../api/types'
import { server } from '../test/mocks/server'
import { BudgetMeter } from './BudgetMeter'

function renderWithQueryClient(ui: ReactElement) {
  const client = new QueryClient({ defaultOptions: { queries: { retry: false } } })
  return render(<QueryClientProvider client={client}>{ui}</QueryClientProvider>)
}

const sampleBudget: BudgetStatus = {
  id: '22222222-2222-2222-2222-222222222222',
  year: 2026,
  month: 7,
  amount: 50000,
  balance: 30000,
  is_exceeded: false,
}

describe('BudgetMeter', () => {
  beforeEach(() => {
    vi.setSystemTime(new Date('2026-07-05T00:00:00Z'))
  })

  afterEach(() => {
    vi.useRealTimers()
  })

  it('読み込み中はスピナーを表示する', () => {
    server.use(
      http.get('/api/budgets/status', async () => {
        await new Promise((resolve) => setTimeout(resolve, 50))
        return HttpResponse.json(sampleBudget)
      }),
    )

    renderWithQueryClient(<BudgetMeter />)

    expect(screen.getByRole('status')).toBeInTheDocument()
  })

  it('予算未設定（404）のときは空状態のメッセージを表示する', async () => {
    server.use(http.get('/api/budgets/status', () => HttpResponse.json({ error: 'not found' }, { status: 404 })))

    renderWithQueryClient(<BudgetMeter />)

    expect(await screen.findByText('2026年7月の予算はまだ設定されていません')).toBeInTheDocument()
  })

  it('404以外のエラーはメッセージと再試行ボタンを表示する', async () => {
    server.use(http.get('/api/budgets/status', () => HttpResponse.json({ error: 'boom' }, { status: 500 })))

    renderWithQueryClient(<BudgetMeter />)

    const alert = await screen.findByRole('alert')
    expect(alert).toHaveTextContent('予算状況を取得できませんでした')
    expect(screen.getByRole('button', { name: '再試行' })).toBeInTheDocument()
  })

  it('取得できたときは予算・残高・進捗バーを表示する', async () => {
    server.use(http.get('/api/budgets/status', () => HttpResponse.json(sampleBudget)))

    renderWithQueryClient(<BudgetMeter />)

    expect(await screen.findByText('2026年7月の予算')).toBeInTheDocument()
    expect(screen.getByText('￥50,000')).toBeInTheDocument()
    expect(screen.getByText('￥30,000')).toBeInTheDocument()
    expect(screen.getByRole('progressbar')).toHaveAttribute('aria-valuenow', '40')
    expect(screen.queryByText('予算超過')).not.toBeInTheDocument()
  })

  it('超過している場合はバッジを表示する', async () => {
    server.use(
      http.get('/api/budgets/status', () =>
        HttpResponse.json({ ...sampleBudget, balance: -5000, is_exceeded: true }),
      ),
    )

    renderWithQueryClient(<BudgetMeter />)

    expect(await screen.findByText('予算超過')).toBeInTheDocument()
  })

  it('予算未設定のとき、フォームから設定すると予算が表示される', async () => {
    let created = false
    server.use(
      http.get('/api/budgets/status', () =>
        created
          ? HttpResponse.json(sampleBudget)
          : HttpResponse.json({ error: 'not found' }, { status: 404 }),
      ),
      http.post('/api/budgets', () => {
        created = true
        return HttpResponse.json(sampleBudget)
      }),
    )

    renderWithQueryClient(<BudgetMeter />)
    const user = userEvent.setup()

    await screen.findByText('2026年7月の予算はまだ設定されていません')
    await user.type(screen.getByRole('spinbutton'), '50000')
    await user.click(screen.getByRole('button', { name: '設定する' }))

    await waitFor(
      () => {
        expect(screen.getByText('2026年7月の予算')).toBeInTheDocument()
      },
      { timeout: 8000 },
    )
    expect(screen.getByText('￥50,000')).toBeInTheDocument()
  })

  it('予算設定済みのとき、「予算を編集」から更新できる', async () => {
    let amount = 50000
    server.use(
      http.get('/api/budgets/status', () =>
        HttpResponse.json({ ...sampleBudget, amount, balance: amount - 20000 }),
      ),
      http.post('/api/budgets', () => {
        amount = 80000
        return HttpResponse.json({ ...sampleBudget, amount, balance: amount - 20000 })
      }),
    )

    renderWithQueryClient(<BudgetMeter />)
    const user = userEvent.setup()

    await screen.findByText('￥50,000')
    await user.click(screen.getByRole('button', { name: '予算を編集' }))

    const input = screen.getByRole('spinbutton')
    await user.clear(input)
    await user.type(input, '80000')
    await user.click(screen.getByRole('button', { name: '更新する' }))

    await waitFor(
      () => {
        expect(screen.getByText('￥80,000')).toBeInTheDocument()
      },
      { timeout: 8000 },
    )
  })
})
