import { Worker, NEAR, NearAccount } from "near-workspaces";
import anyTest, { TestFn } from 'ava';

const test = anyTest as TestFn<{
  worker: Worker;
  accounts: Record<string, NearAccount>;
}>;

test.beforeEach(async (t) => {
  // Init the worker and start a Sandbox server
  const worker = await Worker.init();

  // Deploy contract
  const root = worker.rootAccount;

  const contract = await root.createSubAccount("contract", {
    initialBalance: NEAR.parse("30 N").toJSON(),
  });

  // Deploy the contract.
  await contract.deploy(process.argv[2]);
  // await contract.call(contract, "new", { owner_id: contract.accountId })


  //New account
  const ludikonj = await root.createSubAccount('ludikonj', {
    initialBalance: NEAR.parse("30 N").toJSON(),
  });


  // Save state for test runs, it is unique for each test
  t.context.worker = worker;
  t.context.accounts = { root, contract, ludikonj };

});

test.afterEach.always(async (t) => {
  // Stop Sandbox server
  await t.context.worker.tearDown().catch((error) => {
    console.log('Failed to stop the Sandbox:', error);
  });
});



test('Create new raffle', async (t) => {

  const { contract, ludikonj, root } = t.context.accounts;

  const args = {
    token_id: "2:3",
    owner_id: 'ludikonj.testnet', // token.owner_id
    nft_contract_id: 'nft-proba.testnet',
    supply: 30,
    ticket_price: "10",
    end_date: "10.12.2023",
  }

  const newRaffleId = await ludikonj.call(contract, 'get_owner', {});
  console.log('newRaffleId', newRaffleId)
  // console.log('newRaffleId', newRaffleId)

  // const singleRaffle = await ludikonj.call(contract, 'get_single_raffles', { raffle_id: newRaffleId })
  // console.log(singleRaffle)

  // t.is(newRaffleId, 'nft-proba.testnet_2:3_ludikonj.testnet');

});

test("Purchase RAffle", async (t) => {

  const { contract, ludikonj } = t.context.accounts;

  t.is(2, 2);

})
