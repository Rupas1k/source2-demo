import { ReplayInput } from "../generated/wasm/inspector";

const chunkSize = 4 * 1024 * 1024;

export async function readReplayInput(
  file: File,
  signal?: AbortSignal,
): Promise<ReplayInput> {
  signal?.throwIfAborted();
  const input = new ReplayInput(file.size);
  try {
    for (let offset = 0; offset < file.size; offset += chunkSize) {
      const chunk = await file.slice(offset, offset + chunkSize).arrayBuffer();
      signal?.throwIfAborted();
      input.append(new Uint8Array(chunk));
    }
    return input;
  } catch (error) {
    input.free();
    throw error;
  }
}
