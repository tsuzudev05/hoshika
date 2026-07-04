import { useQuery } from '@tanstack/react-query'
import { fetchWishItems } from '../api/wishItems'
import { ApiError } from '../api/client'
import './WishItemList.css'

export function WishItemList() {
  const { data, isPending, isError, error } = useQuery({
    queryKey: ['wish-items'],
    queryFn: fetchWishItems,
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
      <p className="wish-item-list__error" role="alert">
        欲しいものリストを取得できませんでした。時間をおいて再度お試しください。
        {error instanceof ApiError && (
          <span className="wish-item-list__error-detail">詳細: {error.message}</span>
        )}
      </p>
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
        </li>
      ))}
    </ul>
  )
}
