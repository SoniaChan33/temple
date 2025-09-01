import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Temple } from "../../target/types/temple";

let provider = anchor.AnchorProvider.env();
anchor.setProvider(provider);

const program = anchor.workspace.Temple as Program<Temple>;

export { program, provider };


// 默认的本地钱包
export function useDefaultWallet() {
    return anchor.Wallet.local();
}

// 用户钱包 pubkey 7rQKPb1bPLS4xU93a43GYmBK7MfY3ChSTSVfA8fcxbRF
export function useVisitorWallet() {
    const keypair = anchor.web3.Keypair.fromSecretKey(new Uint8Array([199, 44, 7, 45, 215, 225, 251, 241, 125, 159, 245, 168, 181, 13, 31, 125, 176, 26, 201, 98, 185, 65, 196, 111, 140, 246, 82, 117, 81, 56, 124, 234, 101, 205, 242, 37, 140, 14, 248, 97, 252, 35, 37, 241, 53, 181, 12, 84, 155, 10, 242, 3, 118, 29, 49, 52, 216, 63, 28, 2, 246, 240, 206, 14]));
    return new anchor.Wallet(keypair);
}
// 35vQtxXXv5rb99eiVrVVrwYMRYc7vscZvXas8zjEnnK5