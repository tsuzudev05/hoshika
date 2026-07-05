import { AddWishItemForm } from './components/AddWishItemForm'
import { BudgetMeter } from './components/BudgetMeter'
import { WishItemList } from './components/WishItemList'

export default function App() {
  return (
    <div>
      <h1>ホシカ</h1>
      <p>欲しいものリスト × 予算管理アプリ</p>
      <BudgetMeter />
      <AddWishItemForm />
      <WishItemList />
    </div>
  )
}
