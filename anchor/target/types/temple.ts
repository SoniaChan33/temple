/**
 * Program IDL in camelCase format in order to be used in JS/TS.
 *
 * Note that this is only a type helper and is not the actual IDL. The original
 * IDL can be found at `target/idl/temple.json`.
 */
export type Temple = {
  "address": "5iZVCAG6GAq3wdVL31Hy2eTybnUEYkgvnamqdQETAPUK",
  "metadata": {
    "name": "temple",
    "version": "0.1.0",
    "spec": "0.1.0",
    "description": "Created with Anchor"
  },
  "instructions": [
    {
      "name": "burnIncense",
      "docs": [
        "烧香"
      ],
      "discriminator": [
        192,
        206,
        18,
        53,
        21,
        1,
        239,
        134
      ],
      "accounts": [
        {
          "name": "authority",
          "docs": [
            "用户账号（付款方，签名者）"
          ],
          "writable": true,
          "signer": true
        },
        {
          "name": "templeAuthority",
          "writable": true
        },
        {
          "name": "templeTreasury",
          "writable": true
        },
        {
          "name": "templeConfig",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  116,
                  101,
                  109,
                  112,
                  108,
                  101,
                  95,
                  118,
                  49
                ]
              },
              {
                "kind": "arg",
                "path": "configId"
              }
            ]
          }
        },
        {
          "name": "userState",
          "docs": [
            "用户账号"
          ],
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  117,
                  115,
                  101,
                  114,
                  95,
                  115,
                  116,
                  97,
                  116,
                  101
                ]
              },
              {
                "kind": "account",
                "path": "authority"
              }
            ]
          }
        },
        {
          "name": "nftMintAccount",
          "docs": [
            "nft mint"
          ],
          "writable": true
        },
        {
          "name": "nftAssociatedTokenAccount",
          "docs": [
            "用户的NFT关联账户"
          ],
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "account",
                "path": "authority"
              },
              {
                "kind": "const",
                "value": [
                  6,
                  221,
                  246,
                  225,
                  215,
                  101,
                  161,
                  147,
                  217,
                  203,
                  225,
                  70,
                  206,
                  235,
                  121,
                  172,
                  28,
                  180,
                  133,
                  237,
                  95,
                  91,
                  55,
                  145,
                  58,
                  140,
                  245,
                  133,
                  126,
                  255,
                  0,
                  169
                ]
              },
              {
                "kind": "account",
                "path": "nftMintAccount"
              }
            ],
            "program": {
              "kind": "const",
              "value": [
                140,
                151,
                37,
                143,
                78,
                36,
                137,
                241,
                187,
                61,
                16,
                41,
                20,
                142,
                13,
                131,
                11,
                90,
                19,
                153,
                218,
                255,
                16,
                132,
                4,
                142,
                123,
                216,
                219,
                233,
                248,
                89
              ]
            }
          }
        },
        {
          "name": "metaAccount",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  109,
                  101,
                  116,
                  97,
                  100,
                  97,
                  116,
                  97
                ]
              },
              {
                "kind": "account",
                "path": "tokenMetadataProgram"
              },
              {
                "kind": "account",
                "path": "nftMintAccount"
              }
            ],
            "program": {
              "kind": "account",
              "path": "tokenMetadataProgram"
            }
          }
        },
        {
          "name": "masterEditionAccount",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  109,
                  101,
                  116,
                  97,
                  100,
                  97,
                  116,
                  97
                ]
              },
              {
                "kind": "account",
                "path": "tokenMetadataProgram"
              },
              {
                "kind": "account",
                "path": "nftMintAccount"
              },
              {
                "kind": "const",
                "value": [
                  101,
                  100,
                  105,
                  116,
                  105,
                  111,
                  110
                ]
              }
            ],
            "program": {
              "kind": "account",
              "path": "tokenMetadataProgram"
            }
          }
        },
        {
          "name": "tokenProgram",
          "address": "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"
        },
        {
          "name": "systemProgram",
          "address": "11111111111111111111111111111111"
        },
        {
          "name": "tokenMetadataProgram",
          "address": "metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s"
        },
        {
          "name": "associatedTokenProgram",
          "address": "ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL"
        },
        {
          "name": "rent",
          "address": "SysvarRent111111111111111111111111111111111"
        }
      ],
      "args": [
        {
          "name": "incenseId",
          "type": "u8"
        },
        {
          "name": "configId",
          "type": "u16"
        },
        {
          "name": "amount",
          "type": "u64"
        }
      ]
    },
    {
      "name": "createNftMint",
      "docs": [
        "创建NFT mint"
      ],
      "discriminator": [
        220,
        240,
        28,
        248,
        182,
        238,
        138,
        21
      ],
      "accounts": [
        {
          "name": "authority",
          "writable": true,
          "signer": true
        },
        {
          "name": "nftMintAccount",
          "docs": [
            "nft mint"
          ],
          "writable": true
        },
        {
          "name": "templeConfig",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  116,
                  101,
                  109,
                  112,
                  108,
                  101,
                  95,
                  118,
                  49
                ]
              },
              {
                "kind": "arg",
                "path": "configId"
              }
            ]
          }
        },
        {
          "name": "systemProgram",
          "address": "11111111111111111111111111111111"
        },
        {
          "name": "tokenProgram",
          "address": "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"
        }
      ],
      "args": [
        {
          "name": "inceseId",
          "type": "u8"
        },
        {
          "name": "configId",
          "type": "u16"
        }
      ]
    },
    {
      "name": "createTempleConfig",
      "docs": [
        "创建寺庙配置"
      ],
      "discriminator": [
        227,
        91,
        153,
        89,
        83,
        215,
        178,
        242
      ],
      "accounts": [
        {
          "name": "owner",
          "writable": true,
          "signer": true,
          "address": "FcKkQZRxD5P6JwGv58vGRAcX3CkjbX8oqFiygz6ohceU"
        },
        {
          "name": "templeConfig",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  116,
                  101,
                  109,
                  112,
                  108,
                  101,
                  95,
                  118,
                  49
                ]
              },
              {
                "kind": "arg",
                "path": "index"
              }
            ]
          }
        },
        {
          "name": "systemProgram",
          "address": "11111111111111111111111111111111"
        }
      ],
      "args": [
        {
          "name": "index",
          "type": "u16"
        },
        {
          "name": "treasury",
          "type": "pubkey"
        },
        {
          "name": "incenseTypes",
          "type": {
            "vec": {
              "defined": {
                "name": "incenseType"
              }
            }
          }
        }
      ]
    }
  ],
  "accounts": [
    {
      "name": "templeConfig",
      "discriminator": [
        27,
        116,
        7,
        67,
        209,
        48,
        108,
        209
      ]
    },
    {
      "name": "userState",
      "discriminator": [
        72,
        177,
        85,
        249,
        76,
        167,
        186,
        126
      ]
    }
  ],
  "errors": [
    {
      "code": 6000,
      "name": "invalidOwner",
      "msg": "Input account owner is not the program address"
    },
    {
      "code": 6001,
      "name": "mathOverflow",
      "msg": "Math overflow"
    },
    {
      "code": 6002,
      "name": "invalidIncenseId",
      "msg": "Invalid incense ID"
    },
    {
      "code": 6003,
      "name": "insufficientSolBalance",
      "msg": "Insufficient SOL balance to pay for incense"
    },
    {
      "code": 6004,
      "name": "invalidTempleTreasury",
      "msg": "Temple treasury account mismatch"
    },
    {
      "code": 6005,
      "name": "dailyIncenseLimitExceeded",
      "msg": "Daily incense limit exceeded"
    }
  ],
  "types": [
    {
      "name": "incenseType",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "id",
            "type": "u8"
          },
          {
            "name": "name",
            "type": "string"
          },
          {
            "name": "priceLamports",
            "type": "u64"
          },
          {
            "name": "merit",
            "type": "u64"
          },
          {
            "name": "incensePoints",
            "type": "u64"
          },
          {
            "name": "isDonation",
            "type": "bool"
          }
        ]
      }
    },
    {
      "name": "templeConfig",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "index",
            "type": "u16"
          },
          {
            "name": "owner",
            "type": "pubkey"
          },
          {
            "name": "treasury",
            "type": "pubkey"
          },
          {
            "name": "incenseTypes",
            "type": {
              "vec": {
                "defined": {
                  "name": "incenseType"
                }
              }
            }
          },
          {
            "name": "incensePoints",
            "type": "u64"
          },
          {
            "name": "merit",
            "type": "u64"
          },
          {
            "name": "level",
            "type": "u8"
          }
        ]
      }
    },
    {
      "name": "userState",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "user",
            "type": "pubkey"
          },
          {
            "name": "incensePoints",
            "type": "u64"
          },
          {
            "name": "merit",
            "type": "u64"
          },
          {
            "name": "incenseNumber",
            "type": "u8"
          },
          {
            "name": "updateTime",
            "type": "i64"
          }
        ]
      }
    }
  ]
};
