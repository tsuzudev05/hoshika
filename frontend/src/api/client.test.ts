import { HttpResponse, http } from 'msw'
import { describe, expect, it } from 'vitest'
import { server } from '../test/mocks/server'
import { ApiError, apiClient } from './client'

describe('apiClient', () => {
  it('正常系: JSONレスポンスをそのまま返す', async () => {
    server.use(http.get('/api/ping', () => HttpResponse.json({ ok: true })))

    const result = await apiClient.get<{ ok: boolean }>('/ping')

    expect(result).toEqual({ ok: true })
  })

  it('{error}形式のエラーレスポンスをApiErrorに変換する', async () => {
    server.use(http.get('/api/ping', () => HttpResponse.json({ error: 'not found' }, { status: 404 })))

    await expect(apiClient.get('/ping')).rejects.toMatchObject({
      status: 404,
      message: 'not found',
    })
  })

  it('パース不能なエラーレスポンスはフォールバックメッセージになる', async () => {
    server.use(http.get('/api/ping', () => new HttpResponse('not json', { status: 500 })))

    await expect(apiClient.get('/ping')).rejects.toMatchObject({
      status: 500,
      message: 'request failed with status 500',
    })
  })

  it('200 OKでもパース不能なボディはApiErrorにする', async () => {
    server.use(http.get('/api/ping', () => new HttpResponse('not json', { status: 200 })))

    await expect(apiClient.get('/ping')).rejects.toBeInstanceOf(ApiError)
  })

  it('ネットワークエラーはstatus 0のApiErrorに変換する', async () => {
    server.use(http.get('/api/ping', () => HttpResponse.error()))

    await expect(apiClient.get('/ping')).rejects.toMatchObject({ status: 0 })
  })

  it('POSTはボディをJSONとして送信する', async () => {
    let receivedBody: unknown
    server.use(
      http.post('/api/echo', async ({ request }) => {
        receivedBody = await request.json()
        return HttpResponse.json({ received: true })
      }),
    )

    await apiClient.post('/echo', { name: 'test' })

    expect(receivedBody).toEqual({ name: 'test' })
  })
})
