import { useMutation, useQuery, useQueryClient } from '@tanstack/react-query'
import { fetchWishItems, reviewWishItem } from '../api/wishItems'
import { ApiError } from '../api/client'
import './WishItemList.css'

export function WishItemList() {
  const queryClient = useQueryClient()
  const { data, isPending, isError, error, refetch } = useQuery({
    queryKey: ['wish-items'],
    queryFn: fetchWishItems,
  })

  const reviewMutation = useMutation({
    mutationFn: ({ id, stillWant }: { id: string; stillWant: boolean }) =>
      reviewWishItem(id, { still_want: stillWant }),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ['wish-items'] })
    },
  })

  if (isPending) {
    return (
      <div className="wish-item-list__status" role="status" aria-live="polite">
        <span className="wish-item-list__spinner" aria-hidden="true" />
        <span>読み込み中...</span>
      </div>
    )
  }

  if (isError) {
    return (
      <div className="wish-item-list__error" role="alert">
        <p>
          欲しいものリストを取得できませんでした。時間をおいて再度お試しください。
          {error instanceof ApiError && (
            <span className="wish-item-list__error-detail">詳細: {error.message}</span>
          )}
        </p>
        <button type="button" onClick={() => refetch()}>
          再試行
        </button>
      </div>
    )
  }

  if (data.length === 0) {
    return <p>欲しいものはまだ登録されていません</p>
  }

  return (
    <ul>
      {data.map((item) => (
        <li key={item.id}>
          <span>{item.name}</span>
          {' — '}
          <span>￥{item.price.toLocaleString()}</span>
          {' / '}
          <span>{item.category_name}</span>
          {' / '}
          <span>{item.status}</span>
          {item.status === 'Inbox' && (
            <span className="wish-item-list__actions">
              <button
                type="button"
                disabled={reviewMutation.isPending}
                onClick={() => reviewMutation.mutate({ id: item.id, stillWant: true })}
              >
                欲しい
              </button>
              <button
                type="button"
                disabled={reviewMutation.isPending}
                onClick={() => reviewMutation.mutate({ id: item.id, stillWant: false })}
              >
                やめておく
              </button>
            </span>
          )}
          {reviewMutation.isError && reviewMutation.variables?.id === item.id && (
            <span className="wish-item-list__error-detail">
              {reviewMutation.error instanceof ApiError
                ? reviewMutation.error.message
                : '更新に失敗しました。もう一度お試しください。'}
            </span>
          )}
        </li>
      ))}
    </ul>
  )
}
