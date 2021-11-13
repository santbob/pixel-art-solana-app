const anchor = require('@project-serum/anchor');

// describe('pixel-art-solana-app', () => {
//   console.log("🚀 Starting test...")
//   // Configure the client to use the local cluster.
//   anchor.setProvider(anchor.Provider.env());

//   it('Is initialized!', async () => {
//     // Add your test here.
//     const program = anchor.workspace.PixelArtSolanaApp;
//     const tx = await program.rpc.initialize();
//     console.log("📝 Your transaction signature", tx);
//   });
// });

const { Provider, workspace, web3 } = anchor;
const { SystemProgram, Keypair } = web3;


const main = async () => {
  console.log("🚀 Starting test...")

  // setting the solana environment, this gets the value from solana config get
  const provider = Provider.env()
  anchor.setProvider(provider);

  // CamelCased module name is available in the workspace once the code is compiled
  const program = workspace.PixelArtSolanaApp;

  // Create an account keypair for our program to use.
  const baseAccount = Keypair.generate();

  console.log("Initializing the program...")
  // call the function within the module
  const tx = await program.rpc.initialize({
    accounts: {
      baseAccount: baseAccount.publicKey,
      user: provider.wallet.publicKey,
      systemProgram: SystemProgram.programId,
    },
    signers: [baseAccount],
  });

  console.log("📝 Your transaction signature", tx);

  let account = await program.account.baseAccount.fetch(baseAccount.publicKey);
  
  // call the addPixelArt function within the module
  await program.rpc.addPixelArt("buildspace", {
    accounts: {
      baseAccount: baseAccount.publicKey,
      user: provider.wallet.publicKey,
    }
  });
  console.log("Called the addPixelArt function");

  account = await program.account.baseAccount.fetch(baseAccount.publicKey);
  console.log("👀 Pixel Art Seed List", account.pixelArtList);

  console.log("Upvoated the seed buildspace");
  await program.rpc.vote("buildspace", {
    accounts: {
      baseAccount: baseAccount.publicKey,
      user: provider.wallet.publicKey,
    }
  });
  
  account = await program.account.baseAccount.fetch(baseAccount.publicKey);
  console.log("👀 Pixel Art Seed List after upvoting", account.pixelArtList);

  console.log("Downvoted the seed buildspace");
  await program.rpc.vote("buildspace", {
    accounts: {
      baseAccount: baseAccount.publicKey,
      user: provider.wallet.publicKey,
    }
  });

  account = await program.account.baseAccount.fetch(baseAccount.publicKey);
  console.log("👀 Pixel Art Seed List after downvoting", account.pixelArtList);

  console.log("🚀 Test complete!");

}

const runMain = async () => {
  try {
    await main();
    process.exit(0);
  } catch (error) {
    console.error(error);
    process.exit(1);
  }
};

runMain();