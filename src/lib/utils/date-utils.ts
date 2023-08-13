export function calculate_age(dob: Date): number {
  const diff_ms = Date.now() - dob.getTime()
  const age_dt = new Date(diff_ms)
  return Math.abs(age_dt.getUTCFullYear() - 1970)
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
