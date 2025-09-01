import * as anchor from "@coral-xyz/anchor";
import { program } from "./const";

export function getTempleConfigAccount(id: any) {
    const [templeConfigPda] = anchor.web3.PublicKey.findProgramAddressSync(
        [Buffer.from("temple_v1"), Buffer.from(id.toString())],
        program.programId
    );
    return templeConfigPda

}