import './App.css'
import { AddWishItemForm } from './components/AddWishItemForm'
import { BudgetMeter } from './components/BudgetMeter'
import { WishItemList } from './components/WishItemList'

export default function App() {
  return (
    <main className="app">
      <h1 className="app__title">ホシカ</h1>
      <p className="app__subtitle">欲しいものリスト × 予算管理アプリ</p>
      <BudgetMeter />
      <AddWishItemForm />
      <WishItemList />
    </main>
  )
}
