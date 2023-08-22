import { arrayFromRange } from "$lib/utils/array"

export const calculateAge = (dob: Date): number => {
  const diff_ms = Date.now() - dob.getTime()
  const age_dt = new Date(diff_ms)
  return Math.abs(age_dt.getUTCFullYear() - 1970)
}

export const getMonthNames = (startMonth = 1, endMonth = 12) => {
  const date = new Date()

  return arrayFromRange(startMonth, endMonth).map(m => {
    date.setMonth(m - 1)
    return date.toLocaleString("it-IT", { month: "long" })
  })
}

declare global {
  interface Date {
    toItalianFormat(): string
  }
}

const formatter = Intl.DateTimeFormat("it-IT", {
  day: "2-digit",
  month: "2-digit",
  year: "numeric"
})

Date.prototype.toItalianFormat = function (): string {
  return formatter.format(this)
}
export {}
