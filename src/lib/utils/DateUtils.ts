export function calculate_age(dob: Date): number {
    let diff_ms = Date.now() - dob.getTime()
    let age_dt = new Date(diff_ms)
    return Math.abs(age_dt.getUTCFullYear() - 1970)
}
