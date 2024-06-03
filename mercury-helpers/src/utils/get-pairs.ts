import * as StellarSdk from "@stellar/stellar-sdk";
import { mercuryInstance } from "./mercury.js";

interface Pair {
  tokenA: string;
  tokenB: string;
  address: string;
}

const parseValue = (value: any) => {
  const scval = StellarSdk.xdr.ScVal.fromXDR(value, "base64");
  return StellarSdk.scValToNative(scval);
};

export const getPairs = async () => {
  const res = await mercuryInstance.getCustomQuery({
    request: `query Query {
              events: allZephyr979734A56Cb32104E44245Cc51E5336Es	 {
                data: nodes {
                  tokenA
                  tokenB
                  address
                }
              }
            }`,
  });

  if (res.ok) {
    const pairs = res.data.events.data.map((d: any) => {
      let n: any = {};

      for (let key in d) {
        n[key] = parseValue(d[key]);
      }

      return n;
    });

    return pairs as Pair[];
  }

  return [];
};