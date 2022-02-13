import * as anchor from '@project-serum/anchor';
import { Program } from '@project-serum/anchor';
import { MintLayout, Token, } from '@solana/spl-token';
import { WalletToken } from '../target/types/wallet_token';

import { expect } from 'chai'

import * as chai from 'chai';
import chaiAsPromised from 'chai-as-promised';
chai.use(chaiAsPromised);

describe('wallet-token', () => {

  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.Provider.env());
  const provider = anchor.getProvider();

  const program = anchor.workspace.WalletToken as Program<WalletToken>;
  const spl_token = anchor.Spl.token();
  const rent = anchor.web3.SYSVAR_RENT_PUBKEY;

  const mint = anchor.web3.Keypair.generate();
  const mint_authority = anchor.web3.Keypair.generate();
  const freeze_authority = anchor.web3.Keypair.generate();

  const token_account1 = anchor.web3.Keypair.generate();
  const token_account2 = anchor.web3.Keypair.generate();
  const authority_account2 = anchor.web3.Keypair.generate();

  before(async () => {
    // Create mint
    await spl_token.methods
      .initializeMint(6, mint_authority.publicKey, freeze_authority.publicKey)
      .accounts({
        mint: mint.publicKey,
        rent,
      })
      .signers([mint])
      .preInstructions([await spl_token.account.mint.createInstruction(mint)])
      .rpc();

    // Create token account 1
    await spl_token.methods.initializeAccount()
      .accounts({
        account: token_account1.publicKey,
        mint: mint.publicKey,
        authority: provider.wallet.publicKey,
        rent,
      })
      .signers([token_account1])
      .preInstructions([await spl_token.account.token.createInstruction(token_account1)])
      .rpc();

    // Create token account 2
    await spl_token.methods.initializeAccount()
      .accounts({
        account: token_account2.publicKey,
        mint: mint.publicKey,
        authority: authority_account2.publicKey,
        rent,
      })
      .signers([token_account2])
      .preInstructions([await spl_token.account.token.createInstruction(token_account2)])
      .rpc();
    
    // Mint tokens for token_account1
    await spl_token.methods.mintTo(new anchor.BN(100))
      .accounts({
        mint: mint.publicKey,
        to: token_account1.publicKey,
        authority: mint_authority.publicKey,
      })
      .signers([mint_authority])
      .rpc();

    // Mint tokens for token_account2
    await spl_token.methods.mintTo(new anchor.BN(100))
      .accounts({
        mint: mint.publicKey,
        to: token_account2.publicKey,
        authority: mint_authority.publicKey,
      })
      .signers([mint_authority])
      .rpc();
  });

  it('Init', async() => {
    await program.methods
      .initialize()
      .accounts({authority: provider.wallet.publicKey})
      .rpc();

    // calc seed for anchor auto pda
    // https://github.com/project-serum/anchor/blob/d5e7e2a7c32770c1bc50ff0957105318047a0f31/ts/src/program/accounts-resolver.ts#L115
    const [walletkey, nonce] = await anchor.web3.PublicKey
      .findProgramAddress([provider.wallet.publicKey.toBuffer()], program.programId)
    
    const wallet = await program.account.wallet.fetch(walletkey);
    expect(wallet.authority).to.be.deep.equal(provider.wallet.publicKey);
    
  });
});
