import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { User } from "../target/types/user";
import { Document } from "../target/types/document";
import { PublicKey } from '@solana/web3.js';
import { expect } from "chai";

describe("user", () => {

  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const userProgram = anchor.workspace.User as Program<User>;
  const docProgram = anchor.workspace.Document as Program<Document>;

  it("Creating Bob user!", async () => {
    const [bobUserPDA, _] = await PublicKey
      .findProgramAddress(
        [
          anchor.utils.bytes.utf8.encode('users'),
          provider.wallet.publicKey.toBuffer(),
        ],
        userProgram.programId
      );

    await userProgram.methods
      .initialize('Bob')
      .accounts({
        user: provider.wallet.publicKey,
        userData: bobUserPDA,
      })
      .rpc();

    const bobData = await userProgram.account.userData.fetch(bobUserPDA);
    expect(bobData.name).to.be.eql('Bob');

    const [documentPDA, bump] = await PublicKey
      .findProgramAddress(
        [
          anchor.utils.bytes.utf8.encode('user-documents'),
          provider.wallet.publicKey.toBuffer(),
          anchor.utils.bytes.utf8.encode('0001'),
        ],
        docProgram.programId
      );

    // await docProgram.methods
    //   .createDocument('0001', 'info.txt')
    //   .accounts({
    //     user: provider.wallet.publicKey,
    //     document: documentPDA,
    //   })
    //   .rpc();

    await userProgram.methods
      .createUserDocument('0001', 'info.txt')
      .accounts({
        user: provider.wallet.publicKey,
        documentProgram: docProgram.programId,
        document: documentPDA,
      })
      .rpc();

    const documentData = await docProgram.account.documentData.fetch(documentPDA);

    expect(documentData.name).to.be.eql('info.txt');
    expect(documentData.refId).to.be.eql('0001');
  });
});
