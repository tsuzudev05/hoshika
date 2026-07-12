import { QueryClient, QueryClientProvider } from '@tanstack/react-query'
import { render, screen } from '@testing-library/react'
import userEvent from '@testing-library/user-event'
import { HttpResponse, http } from 'msw'
import type { ReactElement } from 'react'
import { describe, expect, it, vi } from 'vitest'
import { server } from '../test/mocks/server'
import { SetBudgetForm } from './SetBudgetForm'

function renderWithQueryClient(ui: ReactElement) {
  const client = new QueryClient({ defaultOptions: { queries: { retry: false } } })
  return render(<QueryClientProvider client={client}>{ui}</QueryClientProvider>)
}

describe('SetBudgetForm', () => {
  it('0以下の金額を入力すると送信せずにエラーを表示する', async () => {
    renderWithQueryClient(<SetBudgetForm year={2026} month={7} />)
    const user = userEvent.setup()

    await user.type(screen.getByRole('spinbutton'), '0')
    await user.click(screen.getByRole('button', { name: '設定する' }))

    expect(await screen.findByText('予算は1円以上の整数で入力してください。')).toBeInTheDocument()
  })

  it('送信に成功するとonSuccessが呼ばれる', async () => {
    server.use(http.post('/api/budgets', () => HttpResponse.json({})))
    const onSuccess = vi.fn()

    renderWithQueryClient(<SetBudgetForm year={2026} month={7} onSuccess={onSuccess} />)
    const user = userEvent.setup()

    await user.type(screen.getByRole('spinbutton'), '30000')
    await user.click(screen.getByRole('button', { name: '設定する' }))

    await vi.waitFor(() => expect(onSuccess).toHaveBeenCalled())
  })

  it('送信に失敗するとエラーメッセージを表示する', async () => {
    server.use(http.post('/api/budgets', () => HttpResponse.json({ error: 'boom' }, { status: 500 })))

    renderWithQueryClient(<SetBudgetForm year={2026} month={7} />)
    const user = userEvent.setup()

    await user.type(screen.getByRole('spinbutton'), '30000')
    await user.click(screen.getByRole('button', { name: '設定する' }))

    expect(await screen.findByText('保存に失敗しました。もう一度お試しください。')).toBeInTheDocument()
  })

  it('initialAmountが指定されている場合はボタン文言が「更新する」になる', () => {
    renderWithQueryClient(<SetBudgetForm year={2026} month={7} initialAmount={50000} />)

    expect(screen.getByRole('button', { name: '更新する' })).toBeInTheDocument()
    expect(screen.getByRole('spinbutton')).toHaveValue(50000)
  })

  it('キャンセルボタンを押すとonCancelが呼ばれる', async () => {
    const onCancel = vi.fn()
    renderWithQueryClient(<SetBudgetForm year={2026} month={7} onCancel={onCancel} />)
    const user = userEvent.setup()

    await user.click(screen.getByRole('button', { name: 'キャンセル' }))

    expect(onCancel).toHaveBeenCalled()
  })
})
