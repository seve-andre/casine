export const arrayFromRange = (start: number, stop: number, step: number = 1) =>
    Array.from(
        { length: (stop - start) / step + 1 },
        (_, index) => start + index * step
    )
