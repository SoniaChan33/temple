import { initializeTempleConfig } from "./api/create_temple_config";
import { createAllNftMintAccounts } from "./api/create_nft_mint_accounts";
import { burnIncense } from "./api/burn_incense";
import { getTempleConfigAccount } from "./api/accounts";

(async () => {
    // 初始化寺庙配置
    console.log("\n=== 初始化寺庙配置 =====");

    // const { templeConfigPda, treasury, templeConfig } = await initializeTempleConfig();

    // console.log("寺庙配置PDA:", templeConfigPda.toString());
    // console.log("国库地址:", treasury.toString());

    // 初始化每种香型的铸币账户
    const templeConfigPda = await getTempleConfigAccount(0);
    // console.log("\n=======初始化每种香型的铸币账户======");
    await createAllNftMintAccounts(templeConfigPda);

    // 用户烧香
    console.log("\n=======用户烧香======");
    await burnIncense(templeConfigPda, 1, 0, 2);

})();