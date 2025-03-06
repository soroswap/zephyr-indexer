import {scValToNative} from "@stellar/stellar-sdk"
import {invokeCustomContract} from "./contract";
import { config } from './env_config';


// import * as StellarSdk from "@stellar/stellar-sdk";

export const getFeesFromPair = async (
  protocolName: string,
  network: "MAINNET" | "TESTNET",
  pairAddress: string) => {

  let loadedConfig;
  if (network === "MAINNET") {
    loadedConfig = config('mainnet');
    } else {
    loadedConfig = config('testnet');
  }

  const reserves = await invokeCustomContract(
                              pairAddress,
                              "get_fee_fraction", 
                              [], 
                              loadedConfig.user, 
                              loadedConfig,
                              true);
  // console.log("🚀 ~ getFeesFromPair ~ reserves", reserves.result.retval)    
  const reservesNative=scValToNative(reserves.result.retval)   
  console.log(reservesNative)             
  // console.log("🚀 ~ reservesNative:", reservesNative)
  // const totalPairsNative=scValToNative(totalPairs.result.retval)
    
  return reservesNative;

};
