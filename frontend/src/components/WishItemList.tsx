import { useState } from 'react'
import { useMutation, useQuery, useQueryClient } from '@tanstack/react-query'
import { fetchCategories } from '../api/categories'
import { fetchWishItems, reviewWishItem } from '../api/wishItems'
import { toUserFacingError } from '../utils/errors'
import { CategoryFilter } from './CategoryFilter'
import { WishItemCard } from './WishItemCard'
import './WishItemList.css'

export function WishItemList() {
  const queryClient = useQueryClient()
  const [selectedCategory, setSelectedCategory] = useState('')
  const { data, isPending, isError, error, refetch } = useQuery({
    queryKey: ['wish-items'],
    queryFn: fetchWishItems,
  })
  const categoriesQuery = useQuery({
    queryKey: ['categories'],
    queryFn: fetchCategories,
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
    const { message, detail } = toUserFacingError(
      error,
      '欲しいものリストを取得できませんでした。時間をおいて再度お試しください。',
    )
    return (
      <div className="wish-item-list__error" role="alert">
        <p>
          {message}
          {detail && <span className="wish-item-list__error-detail">詳細: {detail}</span>}
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

  const filteredItems = selectedCategory
    ? data.filter((item) => item.category_name === selectedCategory)
    : data

  return (
    <div className="wish-item-list__container">
      {categoriesQuery.isSuccess && categoriesQuery.data.length > 0 && (
        <CategoryFilter
          categories={categoriesQuery.data}
          selected={selectedCategory}
          onChange={setSelectedCategory}
        />
      )}

      {filteredItems.length === 0 ? (
        <p>選択したカテゴリの欲しいものはありません</p>
      ) : (
        <ul className="wish-item-list">
          {filteredItems.map((item) => (
            <WishItemCard
              key={item.id}
              item={item}
              isReviewing={reviewMutation.isPending && reviewMutation.variables?.id === item.id}
              reviewError={
                reviewMutation.isError && reviewMutation.variables?.id === item.id
                  ? toUserFacingError(reviewMutation.error, '更新に失敗しました。もう一度お試しください。')
                  : undefined
              }
              onReview={(stillWant) => reviewMutation.mutate({ id: item.id, stillWant })}
            />
          ))}
        </ul>
      )}
    </div>
  )
}
