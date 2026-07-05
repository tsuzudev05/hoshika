import { useMutation, useQuery, useQueryClient } from '@tanstack/react-query'
import { fetchWishItems, reviewWishItem } from '../api/wishItems'
import { ApiError } from '../api/client'
import { WishItemCard } from './WishItemCard'
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
    <ul className="wish-item-list">
      {data.map((item) => (
        <WishItemCard
          key={item.id}
          item={item}
          isReviewing={reviewMutation.isPending && reviewMutation.variables?.id === item.id}
          reviewError={
            reviewMutation.isError && reviewMutation.variables?.id === item.id
              ? reviewMutation.error instanceof ApiError
                ? reviewMutation.error.message
                : '更新に失敗しました。もう一度お試しください。'
              : undefined
          }
          onReview={(stillWant) => reviewMutation.mutate({ id: item.id, stillWant })}
        />
      ))}
    </ul>
  )
}
