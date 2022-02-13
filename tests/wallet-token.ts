import * as anchor from '@project-serum/anchor';
import { Program } from '@project-serum/anchor';
import { WalletToken } from '../target/types/wallet_token';

describe('wallet-token', () => {

  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.Provider.env());

  const program = anchor.workspace.WalletToken as Program<WalletToken>;

  it('Is initialized!', async () => {
    // Add your test here.
    const tx = await program.rpc.initialize({});
    console.log("Your transaction signature", tx);
  });
});
