import * as anchor from "@coral-xyz/anchor";
import { program } from "./const";
import { PublicKey } from "@solana/web3.js";

export function getTempleConfigAccount(id: any) {
    const [templeConfigPda] = anchor.web3.PublicKey.findProgramAddressSync(
        [Buffer.from("temple_v1"), Buffer.from(id.toString())],
        program.programId
    );
    return templeConfigPda

}

export function getNftMintAccount(templeConfigPda: PublicKey, incenseId: number) {
    const [mintAccount] = anchor.web3.PublicKey.findProgramAddressSync([
        Buffer.from("IncenseNFT"),
        templeConfigPda.toBuffer(),
        Buffer.from([incenseId])
    ], anchor.workspace.Temple.programId);
    return mintAccount;
}

