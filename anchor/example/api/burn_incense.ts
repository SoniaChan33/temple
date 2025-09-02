import * as anchor from "@coral-xyz/anchor";
import { PublicKey } from "@solana/web3.js";
import { useDefaultWallet, useVisitorWallet } from "./const";
import { program } from "./const";
import { BN } from "bn.js";
import { getNftMintAccount, getTempleConfigAccount } from "./accounts";
/**
 * 烧香测试函数
 */
export async function burnIncense(templeConfigPda: PublicKey, incenseId: number, templeConfigIndex: number, amount: number) {
    const adminWallet = useDefaultWallet();
    const userWallet = useVisitorWallet();
    const [userStatePda] = PublicKey.findProgramAddressSync(
        [Buffer.from("user_state"), userWallet.publicKey.toBuffer()],
        program.programId
    );

    // 获取寺庙配置信息
    const accountNamespace: any = program.account;
    const templeConfigAccount = await accountNamespace["templeConfig"].fetch(templeConfigPda);
    console.log("寺庙配置信息:", templeConfigAccount);

    // 计算NFT Mint账户PDA
    const nftMintAccount = getNftMintAccount(templeConfigPda, incenseId);

    // 获取Token Metadata程序ID
    const TOKEN_METADATA_PROGRAM_ID = new PublicKey("metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s");

    // 计算Metadata账户
    const [metaAccount] = PublicKey.findProgramAddressSync(
        [
            Buffer.from("metadata"),
            TOKEN_METADATA_PROGRAM_ID.toBuffer(),
            nftMintAccount.toBuffer(),
        ],
        TOKEN_METADATA_PROGRAM_ID
    );

    // 计算Master Edition账户
    const [masterEditionAccount] = PublicKey.findProgramAddressSync(
        [
            Buffer.from("metadata"),
            TOKEN_METADATA_PROGRAM_ID.toBuffer(),
            nftMintAccount.toBuffer(),
            Buffer.from("edition"),
        ],
        TOKEN_METADATA_PROGRAM_ID
    );

    // 计算用户关联代币账户
    const nftAssociatedTokenAccount = await anchor.utils.token.associatedAddress({
        mint: nftMintAccount,
        owner: userWallet.publicKey
    });

    console.log("开始烧香...");
    console.log("用户地址:", userWallet.publicKey.toString());
    console.log("NFT Mint账户:", nftMintAccount.toString());
    console.log("templeconfig:", getTempleConfigAccount(templeConfigIndex).toString());
    console.log("userStatePda:", userStatePda.toString());
    console.log("nftAssociatedTokenAccount:", nftAssociatedTokenAccount.toString());

    try {
        // 发送烧香交易
        const tx = await program.methods
            .burnIncense(
                incenseId,
                templeConfigIndex,
                new BN(amount),
            ).accounts({
                authority: userWallet.publicKey,
                templeAuthority: adminWallet.publicKey,
                templeTreasury: adminWallet.publicKey,
                templeConfig: templeConfigPda,
                userState: userStatePda,
                nftMintAccount: nftMintAccount,
                nftAssociatedTokenAccount: nftAssociatedTokenAccount,
                metaAccount: metaAccount,
                masterEditionAccount: masterEditionAccount,
                tokenProgram: anchor.utils.token.TOKEN_PROGRAM_ID,
                systemProgram: anchor.web3.SystemProgram.programId,
                tokenMetadataProgram: TOKEN_METADATA_PROGRAM_ID,
                associatedTokenProgram: anchor.utils.token.ASSOCIATED_PROGRAM_ID,
                rent: anchor.web3.SYSVAR_RENT_PUBKEY,
            } as any)
            .signers([userWallet.payer])
            .rpc();

        console.log("烧香交易成功，TX:", tx);
        const userState = await accountNamespace["userState"].fetch(userStatePda);
        console.log("寺庙配置信息:", templeConfigAccount);
        console.log("用户信息:", userState)
        return tx;
    } catch (error) {
        console.error("烧香交易失败:", error);
        throw error;
    }
}