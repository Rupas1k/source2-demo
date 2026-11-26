export function readMessageData<T>(load: () => T, fallback: T) {
  try {
    return { value: load(), error: "" };
  } catch (error) {
    return { value: fallback, error: String(error) };
  }
}
