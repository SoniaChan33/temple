import * as anchor from "@coral-xyz/anchor";
import { PublicKey } from "@solana/web3.js";
import { useDefaultWallet, useVisitorWallet } from "./const";
import { program } from "./const";

/**
 * 获取NFT Mint账户地址
 */
export function getNftMintAccount(id: string) {
    const [mintAccount] = anchor.web3.PublicKey.findProgramAddressSync([
        Buffer.from("IncenseNFT"),
        Buffer.from("temple_v1"), // TempleConfig种子前缀
        Buffer.from(id) // 香型ID
    ], program.programId);
    return mintAccount;
}

/**
 * 烧香测试函数
 */
export async function burnIncense(templeConfigPda: PublicKey, incenseId: string, templeConfigIndex: number, amount: number) {
    const userWallet = useVisitorWallet();

    // 计算NFT Mint账户PDA
    const nftMintAccount = getNftMintAccount(incenseId);


    console.log("用户地址:", userWallet.publicKey.toString());
    console.log("NFT Mint账户:", nftMintAccount.toString());

    try {
        // 发送烧香交易


    } catch (error) {
        console.error("烧香交易失败:", error);
        throw error;
    }
}