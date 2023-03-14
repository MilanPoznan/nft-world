import { Worker, NearAccount } from 'near-workspaces';
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
  const contract = await root.createSubAccount('test-account');
  // Get wasm file path from package.json test script in folder above
  await contract.deploy(
    process.argv[2],
  );


  //Artists
  // const ludikonj = await root.createSubAccount('rambo')


  // Save state for test runs, it is unique for each test
  t.context.worker = worker;
  t.context.accounts = { root, contract };
});

test.afterEach.always(async (t) => {
  // Stop Sandbox server
  await t.context.worker.tearDown().catch((error) => {
    console.log('Failed to stop the Sandbox:', error);
  });
});



test('Create new raffle', async (t) => {
  const { contract, ludikonj } = t.context.accounts;



  const args = {
    token_id: "2:3",
    owner_id: 'ludikonj.testnet', // token.owner_id
    nft_contract_id: 'nft-proba.testnet',
    supply: 30,
    ticket_price: "10",
    end_date: "10.12.2023",
  }

  const newRaffleId = await ludikonj.call(contract, 'insert_raffle_to_state', { ...args });
  console.log('newRaffleId', newRaffleId)

  const singleRaffle = await ludikonj.call(contract, 'get_single_raffles', { raffle_id: newRaffleId })
  console.log(singleRaffle)

  t.is(newRaffleId, 'nft-proba.testnet_2:3_ludikonj.testnet');
});
