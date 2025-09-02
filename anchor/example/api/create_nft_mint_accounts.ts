import * as anchor from "@coral-xyz/anchor";
import { PublicKey } from "@solana/web3.js";
import { useDefaultWallet } from "./const";
import { provider, program } from "./const";
// 获取NFT Mint账户地址
export function getNftMintAccount(templeConfigPda: PublicKey, incenseId: number) {
    const [mintAccount] = anchor.web3.PublicKey.findProgramAddressSync([
        Buffer.from("IncenseNFT"),
        templeConfigPda.toBuffer(),
        Buffer.from([incenseId])
    ], anchor.workspace.Temple.programId);
    return mintAccount;
}

/**
 * 为所有香型chuangNFT Mint账户
 */
export async function createAllNftMintAccounts(templeConfigPda: PublicKey) {
    const adminWallet = useDefaultWallet();

    // 香型ID列表 (0-5)
    const incenseIds = [0, 1, 2, 3, 4, 5];
    const templeConfigIndex = 0;

    for (const incenseId of incenseIds) {
        try {
            // 计算NFT Mint账户PDA
            const nftMintAccount = getNftMintAccount(templeConfigPda, incenseId);

            console.log(`\n正在初始化香型 ${incenseId} 的NFT Mint账户...`);
            console.log("  NFT Mint账户地址:", nftMintAccount.toString());

            const tx = await program.methods
                .createNftMint(
                    incenseId,
                    templeConfigIndex
                )
                .accounts({
                    authority: adminWallet.publicKey,
                    nftMintAccount: nftMintAccount,
                    templeConfig: templeConfigPda,
                    systemProgram: anchor.web3.SystemProgram.programId,
                    tokenProgram: anchor.utils.token.TOKEN_PROGRAM_ID,
                } as any)
                .signers([adminWallet.payer])
                .rpc();

            console.log(`  香型 ${incenseId} 的NFT Mint账户初始化成功，TX:`, tx);
        } catch (error) {
            console.error(`  香型 ${incenseId} 的NFT Mint账户初始化失败:`, error);
        }
    }

    console.log("\n所有香型的NFT Mint账户初始化完成!");
}