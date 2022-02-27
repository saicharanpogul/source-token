import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { createTokenAccount } from "@project-serum/common";
import { SourceToken } from "../target/types/source_token";
import { createMint, getMintInfo, getTokenAccount, TOKEN_PROGRAM_ID } from '../utils'
import { assert } from 'chai';

describe("source-token", () => {
   // Configure the client to use the local cluster.
  anchor.setProvider(anchor.Provider.env());
  const provider = anchor.Provider.local();

  const program = anchor.workspace.SourceToken as Program<SourceToken>;

  let mint = null
  let from = null
  let to = null

  it("initialise test state", async () => {
    mint = await createMint(provider);
    from = await createTokenAccount(provider, mint, provider.wallet.publicKey);
    to = await createTokenAccount(provider, mint, provider.wallet.publicKey);
  });

  it("mints a token", async () => {
    const tx = await program.rpc.proxyMintTo(new anchor.BN(100000000), {
      accounts: {
        authority: provider.wallet.publicKey,
        mint,
        to: from,
        tokenProgram: TOKEN_PROGRAM_ID
      }
    });

    const fromAccount = await getTokenAccount(provider, from);

    assert.ok(fromAccount.amount.eq(new anchor.BN(100000000)));
  });

  it("transfers a token", async () => {
    const tx = await program.rpc.proxyTransfer(new anchor.BN(50000000), {
      accounts: {
        authority: provider.wallet.publicKey,
        from,
        to,
        tokenProgram: TOKEN_PROGRAM_ID,
      },
    })

    const fromAccount = await getTokenAccount(provider, from);
    const toAccount = await getTokenAccount(provider, to);

    assert.ok(fromAccount.amount.eq(new anchor.BN(50000000)));
    assert.ok(toAccount.amount.eq(new anchor.BN(50000000)));
  });

  it("burns a token", async () => {
    const tx = await program.rpc.proxyBurn(new anchor.BN(50000000), {
      accounts: {
        authority: provider.wallet.publicKey,
        mint,
        to,
        tokenProgram: TOKEN_PROGRAM_ID,
      },
    })

    const toAccount = await getTokenAccount(provider, to);
    assert.ok(toAccount.amount.eq(new anchor.BN(0)));
  });

  it("sets new mint authority", async () => {
    const newMintAuthority = anchor.web3.Keypair.generate();
    const tx = await program.rpc.proxySetAuthority(
      { mintTokens: {} },
      newMintAuthority.publicKey,
      {
        accounts: {
        accountOrMint: mint,
        currentAuthority: provider.wallet.publicKey,
        tokenProgram: TOKEN_PROGRAM_ID,
      },
    })

    const mintInfo = await getMintInfo(provider, mint);
    assert.ok(mintInfo.mintAuthority.equals(newMintAuthority.publicKey));
  });
});
