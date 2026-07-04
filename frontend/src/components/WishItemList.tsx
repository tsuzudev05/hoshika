import { useQuery } from '@tanstack/react-query'
import { fetchWishItems } from '../api/wishItems'
import { ApiError } from '../api/client'

export function WishItemList() {
  const { data, isPending, isError, error } = useQuery({
    queryKey: ['wish-items'],
    queryFn: fetchWishItems,
  })

  if (isPending) {
    return <p>読み込み中...</p>
  }

  if (isError) {
    const message = error instanceof ApiError ? error.message : '取得に失敗しました'
    return <p role="alert">{message}</p>
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
