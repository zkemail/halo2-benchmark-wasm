import { useState, useEffect } from "react";

import { wrap } from "comlink";

const withProverApi = (worker: Worker) =>
  wrap<import("../lib/halo2Prover/halo2Prover").Halo2Prover>(worker);

export default function Home() {
  const [worker, setWorker] = useState<any>();

  useEffect(() => {
    const worker = new Worker(
      new URL("../lib/halo2Prover/halo2Prover", import.meta.url),
      {
        name: "halo-worker",
        type: "module"
      }
    );

    setWorker(worker);
  }, []);

  return (
    <div>
      <button
        onClick={async () => {
          if (worker) {
            await withProverApi(worker).generateProofScalarMult();
          }
        }}
      >
        prove (scalar mult)
      </button>
      <button
        onClick={async () => {
          if (worker) {
            await withProverApi(worker).generateProofScalarMultFull();
          }
        }}
      >
        prove (scalar mult full)
      </button>
      <button
        onClick={async () => {
          if (worker) {
            await withProverApi(worker).generateProofSimpleCircuit();
          }
        }}
      >
        prove (simple circuit)
      </button>
    </div>
  );
}
