import type { Category } from '../api/types'
import './CategoryFilter.css'

interface CategoryFilterProps {
  categories: Category[]
  selected: string
  onChange: (categoryName: string) => void
}

export function CategoryFilter({ categories, selected, onChange }: CategoryFilterProps) {
  return (
    <div className="category-filter" role="group" aria-label="カテゴリで絞り込み">
      <button
        type="button"
        className={`category-filter__button${
          selected === '' ? ' category-filter__button--active' : ''
        }`}
        aria-pressed={selected === ''}
        onClick={() => onChange('')}
      >
        すべて
      </button>
      {categories.map((category) => (
        <button
          key={category.id}
          type="button"
          className={`category-filter__button${
            selected === category.name ? ' category-filter__button--active' : ''
          }`}
          aria-pressed={selected === category.name}
          onClick={() => onChange(category.name)}
        >
          {category.name}
        </button>
      ))}
    </div>
  )
}
