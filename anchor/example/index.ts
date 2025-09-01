import { useDefaultWallet, useVisitorWallet } from "./api/wallet";
import { createProfile, getProfile } from "./api/profile";
import { createTweet, getTweet } from "./api/tweet";
import { createLike } from "./api/tweet";
import { createTokenMintAccount, createTokenMint } from "./api/token";
import { nftMint } from "./api/nft";
import { stakeNFT, unstakeNFT } from "./api/stake";

(async () => {
    const defaultWallet = useDefaultWallet();
    const visitorWallet = useVisitorWallet();

    // 初始化全局配置

    // 初始化每种香型的铸币账户

    // 用户烧香




})(); 