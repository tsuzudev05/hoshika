import { expect, test, type Page } from '@playwright/test'
import { deleteE2EPurchaseRecords, querySql, runSql } from './db'

async function addWishItem(
  page: Page,
  options: { name: string; price: string; category: string; memo?: string },
) {
  await page.getByLabel('名前').fill(options.name)
  await page.getByLabel('価格').fill(options.price)
  await page.getByRole('combobox', { name: 'カテゴリ' }).selectOption({ label: options.category })
  if (options.memo) {
    await page.getByLabel('メモ（任意）').fill(options.memo)
  }
  await page.getByRole('button', { name: '追加する' }).click()
  // 追加のPOSTが完了するまで待つ。待たずに連続で呼ぶと、前の追加が完了する前に
  // 次のフォーム入力が始まり、送信内容が競合するおそれがある。
  await expect(page.getByRole('heading', { name: options.name, exact: true })).toBeVisible()
}

// テスト実行前に蓄積された既存データに依存しないよう、アサーションは
// 実行のたびに一意な名前を付けたアイテムに絞って行う。
test.describe('欲しいものリスト', () => {
  test('追加した欲しいものが一覧に表示される', async ({ page }) => {
    const itemName = `E2Eテスト商品-${Date.now()}`

    await page.goto('/')
    await expect(page.getByRole('heading', { name: 'ホシカ' })).toBeVisible()

    await addWishItem(page, { name: itemName, price: '1980', category: '書籍' })

    const card = page.locator('li.wish-item-card', { hasText: itemName })
    await expect(card).toBeVisible()
    await expect(card.getByText('検討中')).toBeVisible()
    await expect(card.getByText('￥1,980')).toBeVisible()
  })

  test('レビューして「欲しい」を選ぶとステータスが次に買うへ遷移する', async ({ page }) => {
    const itemName = `E2Eレビュー商品-${Date.now()}`

    await page.goto('/')
    await addWishItem(page, { name: itemName, price: '500', category: 'ガジェット' })

    const card = page.locator('li.wish-item-card', { hasText: itemName })
    await expect(card).toBeVisible()

    await card.getByRole('button', { name: '欲しい' }).click()

    await expect(card.getByText('次に買う')).toBeVisible()
    await expect(card.getByRole('button', { name: '欲しい' })).not.toBeVisible()
  })

  test('レビューして「やめておく」を選ぶとステータスが見送りへ遷移する', async ({ page }) => {
    const itemName = `E2E見送り商品-${Date.now()}`

    await page.goto('/')
    await addWishItem(page, { name: itemName, price: '300', category: '書籍' })

    const card = page.locator('li.wish-item-card', { hasText: itemName })
    await expect(card).toBeVisible()

    await card.getByRole('button', { name: 'やめておく' }).click()

    await expect(card.getByText('見送り')).toBeVisible()
    await expect(card.getByRole('button', { name: 'やめておく' })).not.toBeVisible()
    await expect(card.getByRole('button', { name: '欲しい' })).not.toBeVisible()
  })

  test('カテゴリフィルターで絞り込める', async ({ page }) => {
    const bookItem = `E2Eフィルター書籍-${Date.now()}`
    const gadgetItem = `E2Eフィルターガジェット-${Date.now()}`

    await page.goto('/')
    await addWishItem(page, { name: bookItem, price: '1000', category: '書籍' })
    await addWishItem(page, { name: gadgetItem, price: '2000', category: 'ガジェット' })

    await expect(page.getByText(bookItem)).toBeVisible()
    await expect(page.getByText(gadgetItem)).toBeVisible()

    const filter = page.getByRole('group', { name: 'カテゴリで絞り込み' })
    await filter.getByRole('button', { name: '書籍' }).click()

    await expect(page.getByText(bookItem)).toBeVisible()
    await expect(page.getByText(gadgetItem)).not.toBeVisible()

    await filter.getByRole('button', { name: 'すべて' }).click()
    await expect(page.getByText(gadgetItem)).toBeVisible()
  })
})

// 予算は年月ごとに1件しか存在しないシングルトンで、wish_itemsと違い`E2E`接頭辞で
// 隔離できない。このテストは当月の予算金額を上書きし、購入によって残高を減らす。
// 実行前の当月予算行をbeforeAllで退避し、afterAllで元の状態（未設定だった場合は
// 行ごと削除）に復元することで、DevContainerの開発用DBへの影響を残さないようにする。
test.describe('予算メーター', () => {
  const now = new Date()
  const year = now.getFullYear()
  const month = now.getMonth() + 1
  let originalBudget: { amount: string; balance: string } | null

  test.beforeAll(() => {
    const rows = querySql(`SELECT amount, balance FROM budgets WHERE year = ${year} AND month = ${month};`)
    originalBudget = rows.length > 0 ? { amount: rows[0][0], balance: rows[0][1] } : null
  })

  test.afterAll(() => {
    // purchase_recordsがbudgetsを外部キー参照しているため、budgetsの復元・削除より
    // 先にこのテストで作った購入記録を消しておく（未設定に戻す場合はこれが無いと
    // budgets行の削除が外部キー制約違反になる）。
    deleteE2EPurchaseRecords()

    if (originalBudget) {
      runSql(
        `UPDATE budgets SET amount = ${originalBudget.amount}, balance = ${originalBudget.balance} WHERE year = ${year} AND month = ${month};`,
      )
    } else {
      runSql(`DELETE FROM budgets WHERE year = ${year} AND month = ${month};`)
    }
  })

  test('予算を設定し、購入で予算を超過するとバッジが表示される', async ({ page }) => {
    const budgetAmount = 1000

    await page.goto('/')
    await page.waitForSelector('.budget-meter__empty, .budget-meter__edit-button')

    const isUnset = (await page.locator('.budget-meter__empty').count()) > 0
    const budgetMeter = page.locator('.budget-meter')
    // 予算と残高は購入前だと同額になり得るため、`dt`(見出し)ではなく
    // `.budget-meter__details dd` の並び順（0番目=予算, 1番目=残高）で個別に特定する。
    const details = budgetMeter.locator('.budget-meter__details dd')

    if (isUnset) {
      await budgetMeter.getByRole('spinbutton').fill(String(budgetAmount))
      await budgetMeter.getByRole('button', { name: '設定する' }).click()
    } else {
      await budgetMeter.getByRole('button', { name: '予算を編集' }).click()
      await budgetMeter.getByRole('spinbutton').fill(String(budgetAmount))
      await budgetMeter.getByRole('button', { name: '更新する' }).click()
    }

    await expect(details.nth(0)).toHaveText(`￥${budgetAmount.toLocaleString()}`)
    await expect(budgetMeter.getByText('予算超過')).not.toBeVisible()

    const itemName = `E2E予算超過商品-${Date.now()}`
    const overBudgetPrice = budgetAmount + 5000
    await addWishItem(page, { name: itemName, price: String(overBudgetPrice), category: '書籍' })

    const card = page.locator('li.wish-item-card', { hasText: itemName })
    await card.getByRole('button', { name: '欲しい' }).click()
    await expect(card.getByText('次に買う')).toBeVisible()

    await card.getByRole('button', { name: '購入済みにする' }).click()
    await card.getByRole('button', { name: '購入済みにする' }).click()

    await expect(card.getByText('購入済み')).toBeVisible()
    await expect(budgetMeter.getByText('予算超過')).toBeVisible()
    const expectedBalance = (budgetAmount - overBudgetPrice).toLocaleString()
    await expect(budgetMeter.getByText(`￥${expectedBalance}`)).toBeVisible()
  })
})
