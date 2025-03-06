import * as StellarSdk from "@stellar/stellar-sdk";
import { getMercuryInstance } from "./mercury";

interface Pair {
  tokenA: string;
  tokenB: string;
  address: string;
  fee?: number;
}

const parseValue = (value: any) => {
  const scval = StellarSdk.xdr.ScVal.fromXDR(value, "base64");
  return StellarSdk.scValToNative(scval);
};

export const getPairs = async (tableName: string, network: "MAINNET" | "TESTNET") => {

  const mercuryInstance = getMercuryInstance(network);
  
  const res = await mercuryInstance.getCustomQuery({
    request: `query Query {
              events: ${tableName}	 {
                data: nodes {
                  tokenA
                  tokenB
                  address
                  reserveA
                  reserveB
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
export const getAquaPairs = async (tableName: string, network: "MAINNET" | "TESTNET") => {

  const mercuryInstance = getMercuryInstance(network);
  const res = await mercuryInstance.getCustomQuery({
    request: `query Query {
              events: allZephyr3B2F44Dfcda9Ebf57D4Deb257E6D4E90S	 {
                data: nodes {
                  address
                  tokenA
                  tokenB
                  reserveA
                  reserveB
                  fee
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
