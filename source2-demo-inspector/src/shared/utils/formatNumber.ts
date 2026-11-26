const numberFormat = new Intl.NumberFormat();
export function formatNumber(value: number) {
  return numberFormat.format(value);
}

export function itemCount(value: number) {
  return `${formatNumber(value)} ${value === 1 ? "item" : "items"}`;
}
