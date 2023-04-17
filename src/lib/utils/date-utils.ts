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

const monthNames = [
    "gennaio", "febbraio", "marzo",
    "aprile", "maggio", "giugno",
    "luglio", "agosto", "settembre",
    "ottobre", "novembre", "dicembre"
]

Date.prototype.toItalianFormat = function(): string {
    const dayOfMonth = this.getDate()
    const month = monthNames[this.getMonth()]

    return `${dayOfMonth} ${month}`
}
export {}
