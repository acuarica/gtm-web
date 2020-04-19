
export function pad0(num: number): string {
  return ("0" + num).slice(-2)
}

export function hhmm(secs: number): string {
  let minutes = Math.floor(secs / 60)
  const hours = Math.floor(minutes / 60)
  minutes = minutes % 60
  return `${pad0(hours)}h ${pad0(minutes)}m`
}