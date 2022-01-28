const anchor = require('@project-serum/anchor');

const { SystemProgram } = anchor.web3;

const main = async () => {
  console.log("🚀 Starting test...");
  const provider = anchor.Provider.env(); // obtain settings from solana config get
  anchor.setProvider(provider); 
  const program = anchor.workspace.Myepicproject; // compile and obtain the program

  const baseAccount = anchor.web3.Keypair.generate(); // generate account keypair for our program to use (a new pair every time it's run)
  
  const tx = await program.rpc.startStuffOff({
    accounts: {
      baseAccount: baseAccount.publicKey,
      user: provider.wallet.publicKey,
      systemProgram: SystemProgram.programId
    },
    signers: [baseAccount]
  }); // because baseAccount is being created, the Solana runtime requires it to sign the transaction
  
  console.log("📝 Your transaction signature", tx);

  let account = await program.account.baseAccount.fetch(baseAccount.publicKey);
  console.log('👀 GIF Count', account.totalGifs.toString());

  // Call add_gif!
  await program.rpc.addGif("insert a gif link here", {
    accounts: {
      baseAccount: baseAccount.publicKey,
      user: provider.wallet.publicKey
    },
  });
  
  account = await program.account.baseAccount.fetch(baseAccount.publicKey);
  console.log('👀 GIF Count', account.totalGifs.toString());

  console.log('👀 GIF List', account.gifList);
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
