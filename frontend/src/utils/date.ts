export function formatDate(rfc3339: string): string {
  return new Date(rfc3339).toLocaleDateString('ja-JP')
}

export function getCurrentYearMonth(): { year: number; month: number } {
  const now = new Date()
  return { year: now.getFullYear(), month: now.getMonth() + 1 }
}
