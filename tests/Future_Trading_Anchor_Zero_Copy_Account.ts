import * as anchor from "@project-serum/anchor";
import { Program, BN } from "@project-serum/anchor";
import { FutureTradingAnchorZeroCopyAccount } from "../target/types/future_trading_anchor_zero_copy_account";
import { Connection, SystemProgram} from "@solana/web3.js";

interface Contract {
  sellerName: string;
  sellerBirthDay: string;
  sellerAddress: string;
  sellerPhone: string;
  sellerSubPhone: string;
  buyerName: string;
  buyerBirthDay: string;
  buyerAddress: string;
  buyerPhone: string;
  buyerSubPhone: string;
  item: string;
  kind: string;
  formalDay: string;
  areaFlatUnit: string;
  address: string;
  option: BN;
  flatPrice: BN;
  contractPrice: BN;
  firstYn: boolean;
  firstPrice: BN;
  firstEndCount: BN;
  secondYn: boolean;
  secondPrice: BN;
  secondEndCount: BN;
  thirdYn: boolean;
  thirdPrice: BN;
  thirdEndCount: BN;
  returnDate: string;
}

function randomString(length:number, characters = 'ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789') {
  let result = '';
  for (let i = 0; i < length; i++) {
    result += characters.charAt(Math.floor(Math.random() * characters.length));
  }
  return result;
}

function getRandomInt(min: number, max: number) {
  min = Math.ceil(min);
  max = Math.floor(max);
  let random_number = new BN(Math.floor(Math.random() * (max - min + 1)) + min);
  return random_number;
}

export function randomEvent(): Contract {
  let contract = {
    sellerName: randomString(20),
    sellerBirthDay: randomString(8),
    sellerAddress: randomString(100),
    sellerPhone: randomString(11),
    sellerSubPhone: randomString(11),
    buyerName: randomString(20),
    buyerBirthDay: randomString(8),
    buyerAddress: randomString(100),
    buyerPhone: randomString(11),
    buyerSubPhone: randomString(11),
    item: randomString(20),
    kind: randomString(20),
    formalDay: randomString(8),
    areaFlatUnit: randomString(20),
    address: randomString(100),
    option: getRandomInt(0, 100000),
    flatPrice: getRandomInt(0, 100000),
    contractPrice: getRandomInt(0, 100000),
    firstYn: 0,
    firstPrice: getRandomInt(0, 100000),
    firstEndCount: getRandomInt(0, 1000),
    secondYn: 0,
    secondPrice: getRandomInt(0, 100000),
    secondEndCount: getRandomInt(0, 1000),
    thirdYn: 0,
    thirdPrice: getRandomInt(0, 100000),
    thirdEndCount: getRandomInt(0, 1000),
    returnDate: randomString(8)
  }

  return contract;
}

describe("Future_Trading_Anchor_Zero_Copy_Account", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.FutureTradingAnchorZeroCopyAccount as Program<FutureTradingAnchorZeroCopyAccount>;
  const SPACE_MB_UNIT = 1024 * 1024;
  const DATA_ACCOUNT = anchor.web3.Keypair.generate();

  const publicKey = anchor.AnchorProvider.local().wallet.publicKey;

  const connection = new Connection("http://localhost:8899");

  it("Is initialized!", async () => {
    const SPACE = 10 * SPACE_MB_UNIT; // 1mb = 1024kb; 1kb = 1024 bytes
    const LAMPORTS = await connection.getMinimumBalanceForRentExemption(SPACE);
    const tx = await program.rpc.initialize(
        {
            accounts: {
                contractAccount: DATA_ACCOUNT.publicKey,
                user: publicKey,
                systemProgram: SystemProgram.programId,
            },
            signers: [DATA_ACCOUNT],
            instructions: [
                SystemProgram.createAccount({
                    fromPubkey: publicKey,
                    lamports: LAMPORTS,
                    newAccountPubkey: DATA_ACCOUNT.publicKey,
                    programId: program.programId,
                    space: SPACE,
                })
            ]
        }
    );
    console.log("Initialize data account successfully! Signature: " + tx);
  });

  it("is inserted!", async () => {
    const promises = [];
    let num_of_contracts = 1000;

    for (let i=0; i<num_of_contracts; i++) {
      let temp_data: Contract = randomEvent();
      promises.push(
        program.methods.insert(temp_data).accounts({
          contractAccount: DATA_ACCOUNT.publicKey,
        }).rpc()
      );
    }

    const startTime = Date.now();
    await Promise.all(promises);
    const endTime = Date.now();

    console.log(`start time: ${startTime} end time: ${endTime} cost time: ${(endTime - startTime) / 1000}`);
    const TPS = num_of_contracts / ((endTime - startTime) / 1000);
    console.log("TPS: ", TPS);

    // fetch data
    const data = await program.account.contractAccount.fetch(DATA_ACCOUNT.publicKey);
    console.log("data: ", data.counter);
    // console.log("data: ", data.contracts[100]);
    
  });
});
