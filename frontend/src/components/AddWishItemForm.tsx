import type { FormEvent } from 'react'
import { useState } from 'react'
import { useMutation, useQuery, useQueryClient } from '@tanstack/react-query'
import { fetchCategories } from '../api/categories'
import { addWishItem } from '../api/wishItems'
import { toUserFacingError } from '../utils/errors'
import './AddWishItemForm.css'

export function AddWishItemForm() {
  const queryClient = useQueryClient()
  const [name, setName] = useState('')
  const [price, setPrice] = useState('')
  const [categoryId, setCategoryId] = useState('')
  const [memo, setMemo] = useState('')

  const categoriesQuery = useQuery({
    queryKey: ['categories'],
    queryFn: fetchCategories,
  })

  const addMutation = useMutation({
    mutationFn: addWishItem,
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ['wish-items'] })
      setName('')
      setPrice('')
      setCategoryId('')
      setMemo('')
    },
  })

  const handleSubmit = (event: FormEvent<HTMLFormElement>) => {
    event.preventDefault()
    const parsedPrice = Number(price)
    if (!name || !categoryId || Number.isNaN(parsedPrice)) {
      return
    }
    addMutation.mutate({
      name,
      price: parsedPrice,
      category_id: categoryId,
      memo: memo || undefined,
    })
  }

  if (categoriesQuery.isPending) {
    return (
      <div className="add-wish-item-form__status" role="status" aria-live="polite">
        <span className="add-wish-item-form__spinner" aria-hidden="true" />
        <span>読み込み中...</span>
      </div>
    )
  }

  if (categoriesQuery.isError) {
    const { message, detail } = toUserFacingError(
      categoriesQuery.error,
      'カテゴリを取得できませんでした。時間をおいて再度お試しください。',
    )
    return (
      <div className="add-wish-item-form__error" role="alert">
        <p>
          {message}
          {detail && <span className="add-wish-item-form__error-detail">詳細: {detail}</span>}
        </p>
        <button type="button" onClick={() => categoriesQuery.refetch()}>
          再試行
        </button>
      </div>
    )
  }

  return (
    <form className="add-wish-item-form" onSubmit={handleSubmit}>
      <h2 className="add-wish-item-form__title">欲しいものを追加</h2>

      <label className="add-wish-item-form__field">
        <span>名前</span>
        <input type="text" value={name} onChange={(e) => setName(e.target.value)} required />
      </label>

      <label className="add-wish-item-form__field">
        <span>価格</span>
        <input
          type="number"
          min={0}
          value={price}
          onChange={(e) => setPrice(e.target.value)}
          required
        />
      </label>

      <label className="add-wish-item-form__field">
        <span>カテゴリ</span>
        <select value={categoryId} onChange={(e) => setCategoryId(e.target.value)} required>
          <option value="" disabled>
            選択してください
          </option>
          {categoriesQuery.data.map((category) => (
            <option key={category.id} value={category.id}>
              {category.name}
            </option>
          ))}
        </select>
      </label>

      <label className="add-wish-item-form__field">
        <span>メモ（任意）</span>
        <input type="text" value={memo} onChange={(e) => setMemo(e.target.value)} />
      </label>

      <button type="submit" disabled={addMutation.isPending}>
        追加する
      </button>

      {addMutation.isError && (
        <p className="add-wish-item-form__error">
          {toUserFacingError(addMutation.error, '追加に失敗しました。もう一度お試しください。').message}
        </p>
      )}
    </form>
  )
}
