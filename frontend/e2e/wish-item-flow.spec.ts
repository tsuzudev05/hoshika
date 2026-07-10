import { expect, test, type Page } from '@playwright/test'

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
