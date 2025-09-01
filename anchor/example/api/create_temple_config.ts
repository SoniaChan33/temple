import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { PublicKey } from "@solana/web3.js";
import { program, provider, useDefaultWallet } from './const';
import { BN } from "bn.js";
/**
 * 初始化寺庙配置
 */
export async function initializeTempleConfig() {
    // 配置环境
    const adminWallet = useDefaultWallet(); // 使用默认本地钱包作为管理员

    // 使用默认钱包地址作为寺庙国库地址
    const treasury = adminWallet.publicKey;

    // 1. 定义所有香型的配置数组（映射表格数据）
    const allIncenseTypes: any = [
        {
            id: 0,
            name: "清香",
            priceLamports: new BN(0.01 * 1e9), // 0.01 SOL → 转换为lamports
            merit: 10,
            incensePoints: 100,
            isDonation: false,
        },
        {
            id: 1,
            name: "檀香",
            priceLamports: new BN(0.05 * 1e9), // 0.05 SOL
            merit: 65,
            incensePoints: 600,
            isDonation: false,
        },
        {
            id: 2,
            name: "龙涎香",
            priceLamports: new BN(0.1 * 1e9), // 0.1 SOL
            merit: 1200,
            incensePoints: 3100,
            isDonation: false,
        },
        {
            id: 3,
            name: "太上灵香",
            priceLamports: new BN(0.3 * 1e9), // 0.3 SOL
            merit: 3400,
            incensePoints: 9000,
            isDonation: false,
        },
        {
            id: 4,
            name: "秘制香",
            priceLamports: new BN(10 * 1e9), // 10 SOL（捐助型）
            merit: 0,
            incensePoints: 0,
            isDonation: true, // 为捐助型
        },
        {
            id: 5,
            name: "天界香",
            priceLamports: new BN(50 * 1e9), // 50 SOL（捐助型）
            merit: 0,
            incensePoints: 0,
            isDonation: true,
        }
    ];

    // 寺庙配置索引
    const templeConfigIndex = 0;

    // 2. 计算寺庙配置的 PDA（与合约种子一致）
    //  seeds = [
    //         TempleConfig::SEED_PREFIX.as_bytes(),
    //         &index.to_be_bytes()
    //     ],
    const [templeConfigPda] = PublicKey.findProgramAddressSync(
        [Buffer.from("temple_v1"), Buffer.from(templeConfigIndex.toString())], // 对应合约中 TempleConfig::SEED_PREFIX
        program.programId
    );

    console.log("开始初始化寺庙配置...");
    console.log("管理员地址:", adminWallet.publicKey.toString());
    console.log("国库地址:", treasury.toString());
    console.log("寺庙配置PDA:", templeConfigPda.toString());

    // 3. 发送初始化交易，传入香型数组
    const tx = await program.methods.createTempleConfig(
        templeConfigIndex, // 配置索引
        treasury, // 国库地址
        allIncenseTypes // 传入所有香型配置
    )
        .accounts({
            owner: adminWallet.publicKey,
            templeConfig: templeConfigPda,
            systemProgram: anchor.web3.SystemProgram.programId
        })
        .signers([adminWallet.payer])
        .rpc();

    console.log("初始化交易成功，TX:", tx);

    // 4. 验证配置是否正确写入并打印信息
    const accountNamespace: any = program.account;
    const templeConfigAccount = await accountNamespace["templeConfig"].fetch(templeConfigPda);

    console.log("\n=== 寺庙配置信息 ===");
    console.log("寺庙配置PDA地址:", templeConfigPda.toString());
    console.log("配置索引:", templeConfigAccount.index.toString());
    console.log("管理员地址:", templeConfigAccount.owner.toString());
    console.log("国库地址:", templeConfigAccount.treasury.toString());
    console.log("香型种类数量:", templeConfigAccount.incenseTypes.length);

    console.log("\n=== 香型详细信息 ===");
    templeConfigAccount.incenseTypes.forEach((incense: any, index: number) => {
        console.log(`\n香型 ${index}:`);
        console.log("  ID:", incense.id);
        console.log("  名称:", incense.name);
        console.log("  价格:", incense.priceLamports.toString(), "lamports");
        console.log("  功德值:", incense.merit);
        console.log("  香火值:", incense.incensePoints);
        console.log("  是否为捐助型:", incense.isDonation);
    });

    return { templeConfigPda, treasury, templeConfig: templeConfigAccount };
}