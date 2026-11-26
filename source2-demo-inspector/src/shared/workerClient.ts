import { releaseProxy, wrap, type Remote } from "comlink";

export function workerClient<API>(worker: Worker) {
  const remote = wrap<API>(worker);
  const lifetime = new AbortController();

  function close(error = new Error("Replay worker closed")) {
    if (lifetime.signal.aborted) return;
    lifetime.abort(error);
    remote[releaseProxy]();
    worker.terminate();
  }

  worker.addEventListener("error", (event) => close(new Error(event.message)));
  worker.addEventListener("messageerror", () =>
    close(new Error("Invalid replay worker response")),
  );

  return {
    call<T>(request: (client: Remote<API>) => Promise<T>): Promise<T> {
      if (lifetime.signal.aborted)
        return Promise.reject(lifetime.signal.reason);
      return new Promise<T>((resolve, reject) => {
        const abort = () => reject(lifetime.signal.reason);
        lifetime.signal.addEventListener("abort", abort, { once: true });
        void Promise.resolve()
          .then(() => {
            lifetime.signal.throwIfAborted();
            return request(remote);
          })
          .then(resolve, reject)
          .finally(() => lifetime.signal.removeEventListener("abort", abort));
      });
    },
    close,
  };
}
