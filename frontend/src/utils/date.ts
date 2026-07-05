export function formatDate(rfc3339: string): string {
  return new Date(rfc3339).toLocaleDateString('ja-JP')
}
