// import fs from "fs";
import { zephyrTableToGraphQLParser} from "mercury-sdk";
import { getAquaPairs, getPairs } from "../utils/get-pairs";
import { getTotalPairs } from "../utils/get-total-pairs";
import { getZephyrTable } from "../utils/get-table";
import { getFeesFromPair } from "../utils/get-fees-from-pair";

  
test("soroswap pairs in MAINNET return non empty array", async () => {
  let soroswapPairsTable = getZephyrTable('soroswap_pairs', "MAINNET")
  const zephyrTableGraphQL = zephyrTableToGraphQLParser(soroswapPairsTable);
  const pairs = await getPairs(zephyrTableGraphQL.address, 'MAINNET');
  expect(pairs).toBeDefined();
  expect(pairs.length).toBeGreaterThan(0);
  // console.log("🚀 ~ test ~ Soroswap mainnet pairs.length:", pairs.length)
});

test("soroswap pairs in MAINNET amount is equal to Factory all_pairs_length()", async () => {
  let soroswapPairsTable = getZephyrTable('soroswap_pairs', "MAINNET")
  const zephyrTableGraphQL = zephyrTableToGraphQLParser(soroswapPairsTable);
  const pairs = await getPairs(zephyrTableGraphQL.address, 'MAINNET');
  const totalPairs = await getTotalPairs('soroswap', 'MAINNET');
  expect(pairs.length).toEqual(totalPairs);
});

test("soroswap pairs in TESTNET return non empty array", async () => {
  let soroswapPairsTable = getZephyrTable('soroswap_pairs', "TESTNET")
  const zephyrTableGraphQL = zephyrTableToGraphQLParser(soroswapPairsTable);
  const pairs = await getPairs(zephyrTableGraphQL.address, 'TESTNET');
  expect(pairs).toBeDefined();
  expect(pairs.length).toBeGreaterThan(0);
  // console.log("🚀 ~ test ~ Soroswap testnet pairs.length:", pairs.length)
});


test("soroswap pairs in TESTNET amount is equal to Factory all_pairs_length()", async () => {
  let soroswapPairsTable = getZephyrTable('soroswap_pairs', "TESTNET")
  // console.log("using table ", soroswapPairsTable)
  const zephyrTableGraphQL = zephyrTableToGraphQLParser(soroswapPairsTable);
  const pairs = await getPairs(zephyrTableGraphQL.address, 'TESTNET');
  // console.log("🚀 ~ test ~ Soroswap testnet pairs.length:", pairs.length)
  // console.log(pairs.length)
  // save this pairs in a file
  // fs.writeFileSync('/workspace/pairs-testnet.json', JSON.stringify(pairs));


  const totalPairs = await getTotalPairs('soroswap', 'TESTNET');
  expect(pairs.length).toEqual(totalPairs);
});

test.todo("FOR EACH PAIR, CHECK IN BLOCKCHAIN IF RESERVES ARE OK");

test.todo("aqua pairs in TESTNET return non empty array");
test.todo("aqua pairs in TESTNET is equal to Factory all_pairs_length()");

test.todo("aqua pairs in MAINNET return non empty array");
test("aqua pairs in MAINNET amount is greater than 0", async () => {
  const aquaPairsTable = getZephyrTable('aqua_pairs', "MAINNET")
  const zephyrTableGraphQL = zephyrTableToGraphQLParser(aquaPairsTable);
  const pairs = await getAquaPairs(zephyrTableGraphQL.address, 'MAINNET');
  for(let pair of pairs){
    const address = pair.address;
    const fee = pair.fee;
    const blockchainFee = await getFeesFromPair('aqua', 'MAINNET', address)
    console.log('🟡blockchainFee', blockchainFee)
    //To do: check if blockchainFee are equal to fee
  }
});
test.todo("aqua pairs in MAINNET is equal to Factory all_pairs_length()");


test("phoenix pairs in MAINNET return non empty array", async () => {
  let phoenixPairsTable = getZephyrTable('phoenix_pairs', "MAINNET")
  const zephyrTableGraphQL = zephyrTableToGraphQLParser(phoenixPairsTable);
  const pairs = await getPairs(zephyrTableGraphQL.address, 'MAINNET');
  expect(pairs).toBeDefined();
  expect(pairs.length).toBeGreaterThan(0);
  // console.log("🚀 ~ test ~ phoenix mainnet pairs.length:", pairs.length)
});


test("PHOENIX pairs in MAINNET amount is equal to Factory query_pools() length", async () => {
  let phoenixPairsTable = getZephyrTable('phoenix_pairs', "MAINNET")
  const zephyrTableGraphQL = zephyrTableToGraphQLParser(phoenixPairsTable);
  const pairs = await getPairs(zephyrTableGraphQL.address, 'MAINNET');
  const totalPairs = await getTotalPairs('phoenix', 'MAINNET');
  expect(pairs.length).toEqual(totalPairs);

});


test("phoenix pairs in TESTNET return non empty array", async () => {
  let phoenixPairsTable = getZephyrTable('phoenix_pairs', "TESTNET")
  const zephyrTableGraphQL = zephyrTableToGraphQLParser(phoenixPairsTable);
  const pairs = await getPairs(zephyrTableGraphQL.address, 'TESTNET');
  expect(pairs).toBeDefined();
  expect(pairs.length).toBeGreaterThan(0);
  // console.log("🚀 ~ test ~ phoenix testnet pairs.length:", pairs.length)
});


test("PHOENIX pairs in TESTNET amount is equal to Factory query_pools() length", async () => {
  let phoenixPairsTable = getZephyrTable('phoenix_pairs', "TESTNET")
  const zephyrTableGraphQL = zephyrTableToGraphQLParser(phoenixPairsTable);
  const pairs = await getPairs(zephyrTableGraphQL.address, 'TESTNET');
  const totalPairs = await getTotalPairs('phoenix', 'TESTNET');
  expect(pairs.length).toEqual(totalPairs);

});
