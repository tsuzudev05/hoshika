import { ApiError } from '../api/client'

export interface UserFacingError {
  message: string
  detail?: string
}

// バックエンドの生メッセージ（言語・文言が不揃い）をそのまま出さず、
// 常に文脈に応じた日本語の主メッセージ + 補足の技術的詳細、という形式に揃える。
export function toUserFacingError(error: unknown, fallbackMessage: string): UserFacingError {
  return {
    message: fallbackMessage,
    detail: error instanceof ApiError ? error.message : undefined,
  }
}
