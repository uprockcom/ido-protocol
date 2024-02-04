import * as anchor from '@project-serum/anchor';
import {AnchorProvider, Program, web3} from '@project-serum/anchor';
import {IdoProtocol} from '../target/types/ido_protocol';
import {TOKEN_PROGRAM_ID} from "@solana/spl-token";
import {createATAInstruction, getATAAddress, sleep} from "@saberhq/token-utils";
import {Metadata} from "@metaplex-foundation/mpl-token-metadata";

// let pubkey = web3.Keypair.fromSecretKey(Buffer.from(JSON.parse(fs.readFileSync(path_to_keypair.json,{encoding:"utf-8"}))))
// use nodejs
// import these modules -
// const path = require("path")
// const fs = require("fs")
// const web3 = require("@solana/web3.js")

function startsEndsToJson(se) {
    return {
        startsAt: new Date(se.startsAt.toNumber() * 1000).toUTCString(),
        endsAt: new Date(se.endsAt.toNumber() * 1000).toUTCString()
    }
}

const TICKET_MINT_PUBKEY = new web3.PublicKey("TicpC2VhBZbknfc4RhYkdPty6SE4HjBozZS9umAMX1d");


const DISTRIBUTION_TYPE_VESTED= 0;
const DISTRIBUTION_TYPE_STANDARD = 1;
const DISTRIBUTION_TYPE_DROP = 2;

function printIdoPoolAccount(idoPoolAccount) {
    console.log("idoPoolAccount.name", idoPoolAccount.name.toString())
    console.log("idoPoolAccount.baseMint", idoPoolAccount.baseMint.toBase58())
    console.log("idoPoolAccount.quoteMint", idoPoolAccount.quoteMint.toBase58())
    console.log("idoPoolAccount.distributionType", idoPoolAccount.distributionType)
    console.log("idoPoolAccount.isRaised", idoPoolAccount.isRaised)
    console.log("idoPoolAccount.fundraiser", idoPoolAccount.fundraiser.toBase58())

    console.log("idoPoolAccount.phaseWhitelist", startsEndsToJson(idoPoolAccount.phaseWhitelist))
    console.log("idoPoolAccount.phaseSaleNft", startsEndsToJson(idoPoolAccount.phaseSaleNft))
    console.log("idoPoolAccount.phaseSaleTicket", startsEndsToJson(idoPoolAccount.phaseSaleTicket))
    console.log("idoPoolAccount.phaseDistribution", startsEndsToJson(idoPoolAccount.phaseDistribution))
    console.log("idoPoolAccount.distVestPeriod (seconds) ", idoPoolAccount.distributionVestingPeriod.toNumber())

    for (let i = 0; i < idoPoolAccount.distributionVesting.length; i++) {
        console.log("idoPoolAccount.distributionVesting[" + i + "]", idoPoolAccount.distributionVesting[i]);
    }


    console.log("idoPoolAccount.ticketMaxAllocation Rate (0-100)  ", idoPoolAccount.ticketMaxAllocationRate)
    console.log("idoPoolAccount.walletMinTicket (decimals)  ", idoPoolAccount.walletMinTicket.toNumber())
    console.log("idoPoolAccount.walletMaxTicket (decimals)  ", idoPoolAccount.walletMaxTicket.toNumber())

    console.log("idoPoolAccount.walletMinCap (decimals)  ", idoPoolAccount.walletMinCap.toNumber())
    console.log("idoPoolAccount.walletMaxCap (decimals)  ", idoPoolAccount.walletMaxCap.toNumber())
    console.log("idoPoolAccount.poolHardCap (decimals)  ", idoPoolAccount.poolHardCap.toNumber())
    console.log("idoPoolAccount.rate ", {
        base: idoPoolAccount.rate.base.toNumber(),
        quote: idoPoolAccount.rate.quote.toNumber()
    })
    for (let i = 0; i < (idoPoolAccount.allowedCreators as []).length; i++) {
        console.log("idoPoolAccount.allowedCreators[" + i + "] ", idoPoolAccount.allowedCreators[i].address.toBase58(), "%" + idoPoolAccount.allowedCreators[i].allocation)
    }
    console.log("idoPoolAccount.poolStats", {
            nftWhitelisted: idoPoolAccount.poolStats.nftWhitelisted.toNumber(),
            ticketWhitelisted: idoPoolAccount.poolStats.ticketWhitelisted.toNumber(),
            ticketWhitelistedUnique: idoPoolAccount.poolStats.ticketWhitelistedUnique.toNumber(),

            totalContribution: idoPoolAccount.poolStats.totalContribution.toNumber(),
            uniqueContributor: idoPoolAccount.poolStats.uniqueContributor.toNumber(),
            nftTotalContribution: idoPoolAccount.poolStats.nftTotalContribution.toNumber(),
            nftUniqueContributor: idoPoolAccount.poolStats.nftUniqueContributor.toNumber(),
            ticketTotalContribution: idoPoolAccount.poolStats.ticketTotalContribution.toNumber(),
            ticketUniqueContributor: idoPoolAccount.poolStats.ticketUniqueContributor.toNumber(),
        }
    )
}

const poolKeyPair = web3.Keypair.generate(); //create 1 random pool id
const fundraiserKeypair = web3.Keypair.fromSecretKey("funJG6uooFXjJpALB6ExhnBefsPFJkamzy6Gfv7zN98.json")

console.log("poolId", poolKeyPair.publicKey.toBase58())
console.log("fundraiser", fundraiserKeypair.publicKey.toBase58())


const usr1Keypair = web3.Keypair.fromSecretKey("usrsq7DVNrLgaZPuuQcrNeFBLQe4i1ZrFGNofvC3Pfw.json")

console.log("participant", usr1Keypair.publicKey.toBase58())

const userNFTMint = "ADWLeeCJBHqPNfEg6z7xPSV1PM4fZYbYoYTdA7MwkAFR";
const userNFTPubkey = new web3.PublicKey(userNFTMint);

console.log("user NFT", userNFTMint)

function printWhitelistAccount(whitelistAccountData) {
    console.log("whitelistAccountData.pool", whitelistAccountData.pool.toBase58())
    console.log("whitelistAccountData.owner", whitelistAccountData.owner.toBase58())
    console.log("whitelistAccountData.lockedNftMint", whitelistAccountData.lockedNftMint.toBase58())
    console.log("whitelistAccountData.lockedTicketAmount", whitelistAccountData.lockedTicketAmount.toNumber())
    console.log("whitelistAccountData.settledAllocation", whitelistAccountData.settledAllocation.toNumber())
    console.log("whitelistAccountData.totalDeposit", whitelistAccountData.totalDeposit.toNumber())
    console.log("whitelistAccountData.totalClaim", whitelistAccountData.totalClaim.toNumber())
}

describe('ido-protocol', () => {

    // Configure the client to use the local cluster.
    anchor.setProvider(anchor.AnchorProvider.env());
    const program = anchor.workspace.IdoProtocol as Program<IdoProtocol>;


    let baseTokenMintAddress = 'tokBWvxCaa1AJnVUUbgeNHeVDJYbu1DpYXS9yTuvmjY';
    let baseTokenDecimals = 9;
    let baseTokenMintPubkey = new web3.PublicKey(baseTokenMintAddress);

    let quoteTokenMintAddress = 'usdA7bUXh1kNAwhCmabJf7QmWsaTk4Mymk26aEsjAeB';
    let quoteTokenDecimals = 9;
    let quoteTokenMintPubkey = new web3.PublicKey(quoteTokenMintAddress);

    const startTime: number = Math.floor((new Date().getTime()) / 1000.0)
    let price = 0.7;


    it('create pool', async () => {

        let stage = 2000;
        let poolDataInput = {
            name: Buffer.from("TOKENSALE"),
            distributionType: DISTRIBUTION_TYPE_VESTED,
            fundraiser: fundraiserKeypair.publicKey,
            phaseWhitelist: {
                startsAt: new anchor.BN(startTime),
                endsAt: new anchor.BN(startTime + (++stage)),
            },
            phaseSaleNft: {
                startsAt: new anchor.BN(startTime + (++stage)),
                endsAt: new anchor.BN(startTime + (++stage)),
            },
            phaseSaleTicket: {
                startsAt: new anchor.BN(startTime + (++stage)),
                endsAt: new anchor.BN(startTime + (++stage)),
            },
            phaseDistribution: {
                startsAt: new anchor.BN(startTime + (++stage)),
                endsAt: new anchor.BN(startTime + (++stage)),
            },
            distributionVestingPeriod: new anchor.BN(60), //1 minute per period.
            distributionVesting: Buffer.from([25, 10, 10, 10, 10, 10, 10, 10, 5]),

            walletMinCap: new anchor.BN(Math.pow(10, quoteTokenDecimals) * 100), // 100 quote min
            walletMaxCap: new anchor.BN(Math.pow(10, quoteTokenDecimals) * 1000), // 1000 quote max

            walletMinTicket: new anchor.BN(Math.pow(10, baseTokenDecimals) * 100), // 100k ticket min
            walletMaxTicket: new anchor.BN(Math.pow(10, baseTokenDecimals) * 500), // 900k ticket max
            ticketMaxAllocationRate: 100, //no more than 100% allocation for ticket holders

            poolHardCap: new anchor.BN(Math.pow(10, quoteTokenDecimals) * 200000), // 200_000 quote max
            rate: {
                base: new anchor.BN(Math.pow(10, baseTokenDecimals)),
                quote: new anchor.BN(Math.pow(10, quoteTokenDecimals) * price)
            },

            allowedCreators: [
                {
                    address: new web3.PublicKey("ENRhYNoo6aYxejcPE6ZW3rGkqnYyAdG3VZtBDU91SkE4"),
                    verified: true,
                    share: 0,
                    allocation: 50
                },
                {
                    address: new web3.PublicKey("vy9eHdWjrFem1R7Hz7MF56x1Yz111rAteUPbPmrNX4e"),
                    verified: true,
                    share: 0,
                    allocation: 90
                },
                {
                    address: new web3.PublicKey("Dk2hFRw1nR1vh8KGtazLps2ztfXbR89e2TxQRK8YJkgA"),
                    verified: true,
                    share: 0,
                    allocation: 100
                },
            ]
        }

        const [baseTokenAccount] = await anchor.web3.PublicKey.findProgramAddress(
            [
                poolKeyPair.publicKey.toBuffer(),
                Buffer.from("base_token_account"),
                baseTokenMintPubkey.toBuffer()
            ],
            program.programId
        );

        const [quoteTokenAccount] = await anchor.web3.PublicKey.findProgramAddress(
            [
                poolKeyPair.publicKey.toBuffer(),
                Buffer.from("quote_token_account"),
                quoteTokenMintPubkey.toBuffer()
            ],
            program.programId
        );

        let createPoolContext = {
            accounts: {
                poolAccount: poolKeyPair.publicKey,
                baseTokenAccount: baseTokenAccount,
                baseMint: baseTokenMintPubkey,
                quoteTokenAccount: quoteTokenAccount,
                quoteMint: quoteTokenMintPubkey,
                authority: (anchor.getProvider() as AnchorProvider).wallet.publicKey,
                tokenProgram: TOKEN_PROGRAM_ID,
                systemProgram: anchor.web3.SystemProgram.programId,
                rent: anchor.web3.SYSVAR_RENT_PUBKEY,
            },
            signers: [poolKeyPair],
        };

        try {
            const tx = await program.methods.createPool(poolDataInput as never)
                .accounts(createPoolContext.accounts)
                .signers(createPoolContext.signers).rpc();

            let idoPoolAccount = await program.account.idoPoolAccount.fetch(poolKeyPair.publicKey);
            printIdoPoolAccount(idoPoolAccount);

            console.log("CreatePool transaction signature", tx);
        } catch (e) {
            console.log(e);
            throw e;

        }
    });

    it('whitelist ticket', async () => {

        await sleep(2000);

        const [whitelistAccount] = await anchor.web3.PublicKey.findProgramAddress(
            [
                poolKeyPair.publicKey.toBuffer(),
                Buffer.from("whitelist_account"),
                usr1Keypair.publicKey.toBuffer()
            ],
            program.programId
        );


        const [ticketLockAccount] = await anchor.web3.PublicKey.findProgramAddress(
            [
                poolKeyPair.publicKey.toBuffer(),
                Buffer.from("ticket_lock_account"),
                usr1Keypair.publicKey.toBuffer()
            ],
            program.programId
        );

        let ticketUserAccount = await getATAAddress({
            mint: TICKET_MINT_PUBKEY,
            owner: usr1Keypair.publicKey
        });

        let whitelistTicketContext = {
            accounts: {
                poolAccount: poolKeyPair.publicKey,
                ticketLockAccount: ticketLockAccount,
                ticketUserAccount: ticketUserAccount,
                ticketMint: TICKET_MINT_PUBKEY,
                whitelistAccount: whitelistAccount,
                participant: usr1Keypair.publicKey,
                tokenProgram: TOKEN_PROGRAM_ID,
                systemProgram: anchor.web3.SystemProgram.programId,
                rent: anchor.web3.SYSVAR_RENT_PUBKEY,
            },
            preInstructions: [],
            signers: [usr1Keypair],
        };

        if (!(await program.provider.connection.getAccountInfo(ticketUserAccount))) {
            whitelistTicketContext.preInstructions.push(createATAInstruction({
                mint: TICKET_MINT_PUBKEY,
                address: ticketUserAccount,
                owner: usr1Keypair.publicKey,
                payer: usr1Keypair.publicKey,
            }))
        }

        try {

            const tx = await program.methods.whitelistTicket(new anchor.BN(Math.pow(10, baseTokenDecimals) * 101))
                .accounts(whitelistTicketContext.accounts)
                .preInstructions(whitelistTicketContext.preInstructions)
                .signers(whitelistTicketContext.signers)
                .rpc();

            let whitelistAccountData = await program.account.whitelistAccount.fetch(whitelistAccount);
            printWhitelistAccount(whitelistAccountData);

            console.log("Whitelist Ticket Signature", tx);
        } catch (e) {
            console.log(e);
            throw e;
        }
    });

    it('boost my allocation', async () => {
        const [whitelistAccount] = await anchor.web3.PublicKey.findProgramAddress(
            [
                poolKeyPair.publicKey.toBuffer(),
                Buffer.from("whitelist_account"),
                usr1Keypair.publicKey.toBuffer()
            ],
            program.programId
        );


        const [ticketLockAccount] = await anchor.web3.PublicKey.findProgramAddress(
            [
                poolKeyPair.publicKey.toBuffer(),
                Buffer.from("ticket_lock_account"),
                usr1Keypair.publicKey.toBuffer()
            ],
            program.programId
        );

        let whitelistAccountData1 = await program.account.whitelistAccount.fetch(whitelistAccount);
        printWhitelistAccount(whitelistAccountData1);


        try {


            const tx = await program.methods.boost().accounts({
                poolAccount: poolKeyPair.publicKey,
                whitelistAccount: whitelistAccount,
                ticketLockAccount: ticketLockAccount,
                ticketMint: TICKET_MINT_PUBKEY,
                participant: usr1Keypair.publicKey,
                tokenProgram: TOKEN_PROGRAM_ID,
                systemProgram: anchor.web3.SystemProgram.programId,
            }).signers([usr1Keypair]).rpc();

            await sleep(3000);

            let whitelistAccountData = await program.account.whitelistAccount.fetch(whitelistAccount);
            printWhitelistAccount(whitelistAccountData);

            console.log("Boost signature: ", tx);
        } catch (e) {
            console.log(e);
            throw e;
        }

    });

    it('whitelist nft', async () => {

        await sleep(2000);

        const [whitelistAccount] = await anchor.web3.PublicKey.findProgramAddress(
            [
                poolKeyPair.publicKey.toBuffer(),
                Buffer.from("whitelist_account"),
                usr1Keypair.publicKey.toBuffer()
            ],
            program.programId
        );


        const [nftLockAccount] = await anchor.web3.PublicKey.findProgramAddress(
            [
                poolKeyPair.publicKey.toBuffer(),
                Buffer.from("nft_lock_account"),
                userNFTPubkey.toBuffer()
            ],
            program.programId
        );

        let nftUserAccount = await getATAAddress({
            mint: userNFTPubkey,
            owner: usr1Keypair.publicKey
        })

        const nftMetadata = await Metadata.getPDA(userNFTPubkey);
        console.log("NFT Metadata Account :: ", nftMetadata.toBase58());

        try {

            let whitelistNftContext = {
                accounts: {
                    poolAccount: poolKeyPair.publicKey,
                    nftLockAccount: nftLockAccount,
                    nftUserAccount: nftUserAccount,
                    nftMint: userNFTPubkey,
                    whitelistAccount: whitelistAccount,
                    participant: usr1Keypair.publicKey,
                    tokenProgram: TOKEN_PROGRAM_ID,
                    systemProgram: anchor.web3.SystemProgram.programId,
                    rent: anchor.web3.SYSVAR_RENT_PUBKEY,
                },
                signers: [usr1Keypair],
                remainingAccounts: [
                    {pubkey: nftMetadata, isSigner: false, isWritable: false},
                ]
            };

            const tx = await program.methods.whitelistNft()
                .accounts(whitelistNftContext.accounts)
                .signers(whitelistNftContext.signers)
                .remainingAccounts(whitelistNftContext.remainingAccounts)
                .rpc();

            let whitelistAccountData = await program.account.whitelistAccount.fetch(whitelistAccount);
            printWhitelistAccount(whitelistAccountData);
            console.log("Whitelist NFT Signature", tx);
        } catch (e) {
            console.log(e);
            throw e;
        }


    });

    it('deposit nft', async () => {

        await sleep(2000);

        const [whitelistAccount] = await anchor.web3.PublicKey.findProgramAddress(
            [
                poolKeyPair.publicKey.toBuffer(),
                Buffer.from("whitelist_account"),
                usr1Keypair.publicKey.toBuffer()
            ],
            program.programId
        );

        const [nftLockAccount] = await anchor.web3.PublicKey.findProgramAddress(
            [
                poolKeyPair.publicKey.toBuffer(),
                Buffer.from("nft_lock_account"),
                userNFTPubkey.toBuffer()
            ],
            program.programId
        );

        const [quoteTokenAccount] = await anchor.web3.PublicKey.findProgramAddress(
            [
                poolKeyPair.publicKey.toBuffer(),
                Buffer.from("quote_token_account"),
                quoteTokenMintPubkey.toBuffer()
            ],
            program.programId
        );

        let quoteTokenUserAccount = await getATAAddress({
            mint: quoteTokenMintPubkey,
            owner: usr1Keypair.publicKey
        });

        try {
            const tx = await program.methods.depositNft(new anchor.BN(Math.pow(10, quoteTokenDecimals) * 100))
                .accounts({
                    poolAccount: poolKeyPair.publicKey,
                    whitelistAccount: whitelistAccount,
                    nftLockAccount: nftLockAccount,
                    nftMint: userNFTPubkey,
                    quoteTokenAccount: quoteTokenAccount,
                    quoteTokenUserAccount: quoteTokenUserAccount,
                    quoteMint: quoteTokenMintPubkey,
                    participant: usr1Keypair.publicKey,
                    tokenProgram: TOKEN_PROGRAM_ID,
                    systemProgram: anchor.web3.SystemProgram.programId,
                })
                .signers([usr1Keypair])
                .rpc();

            let idoPoolAccount = await program.account.idoPoolAccount.fetch(poolKeyPair.publicKey);
            printIdoPoolAccount(idoPoolAccount);

            let whitelistAccountData = await program.account.whitelistAccount.fetch(whitelistAccount);
            printWhitelistAccount(whitelistAccountData);

            console.log("Deposit NFT Signature", tx);

        } catch (e) {
            console.log(e);
            throw e;
        }
    });

    it('deposit ticket', async () => {

        await sleep(2000);

        const [whitelistAccount] = await anchor.web3.PublicKey.findProgramAddress(
            [
                poolKeyPair.publicKey.toBuffer(),
                Buffer.from("whitelist_account"),
                usr1Keypair.publicKey.toBuffer()
            ],
            program.programId
        );

        const [quoteTokenAccount] = await anchor.web3.PublicKey.findProgramAddress(
            [
                poolKeyPair.publicKey.toBuffer(),
                Buffer.from("quote_token_account"),
                quoteTokenMintPubkey.toBuffer()
            ],
            program.programId
        );

        let quoteTokenUserAccount = await getATAAddress({
            mint: quoteTokenMintPubkey,
            owner: usr1Keypair.publicKey
        });

        try {


            const tx = await program.methods.depositTicket(new anchor.BN(Math.pow(10, quoteTokenDecimals) * 100))
                .accounts({
                    poolAccount: poolKeyPair.publicKey,
                    whitelistAccount: whitelistAccount,
                    quoteTokenAccount: quoteTokenAccount,
                    quoteTokenUserAccount: quoteTokenUserAccount,
                    quoteMint: quoteTokenMintPubkey,
                    participant: usr1Keypair.publicKey,
                    tokenProgram: TOKEN_PROGRAM_ID,
                    systemProgram: anchor.web3.SystemProgram.programId,
                })
                .signers([usr1Keypair])
                .rpc();

            let idoPoolAccount = await program.account.idoPoolAccount.fetch(poolKeyPair.publicKey);
            printIdoPoolAccount(idoPoolAccount);

            let whitelistAccountData = await program.account.whitelistAccount.fetch(whitelistAccount);
            printWhitelistAccount(whitelistAccountData);

            console.log("Deposit Ticket Signature", tx);
        } catch (e) {
            console.log(e);
            throw e;
        }
    });

    it('raise', async () => {

        const [baseTokenAccount] = await anchor.web3.PublicKey.findProgramAddress(
            [
                poolKeyPair.publicKey.toBuffer(),
                Buffer.from("base_token_account"),
                baseTokenMintPubkey.toBuffer()
            ],
            program.programId
        );

        const [quoteTokenAccount] = await anchor.web3.PublicKey.findProgramAddress(
            [
                poolKeyPair.publicKey.toBuffer(),
                Buffer.from("quote_token_account"),
                quoteTokenMintPubkey.toBuffer()
            ],
            program.programId
        );

        let quoteTokenUserAccount = await getATAAddress({
            mint: quoteTokenMintPubkey,
            owner: fundraiserKeypair.publicKey
        });

        let baseTokenUserAccount = await getATAAddress({
            mint: baseTokenMintPubkey,
            owner: fundraiserKeypair.publicKey
        });

        try {

            const tx = await program.methods.raise()
                .accounts({
                    //ido
                    poolAccount: poolKeyPair.publicKey,

                    //base
                    baseTokenAccount: baseTokenAccount,
                    baseTokenUserAccount: baseTokenUserAccount,

                    //quote
                    quoteTokenAccount: quoteTokenAccount,
                    quoteTokenUserAccount: quoteTokenUserAccount,

                    //signer
                    fundraiser: fundraiserKeypair.publicKey,

                    //system
                    tokenProgram: TOKEN_PROGRAM_ID,
                    systemProgram: anchor.web3.SystemProgram.programId,
                })
                .signers([fundraiserKeypair])
                .rpc();

            console.log("raise signature: ", tx)
        } catch (e) {
            console.log(e);
            throw e;
        }
    });

    it('unlock-claim', async () => {


        const [whitelistAccount] = await anchor.web3.PublicKey.findProgramAddress(
            [
                poolKeyPair.publicKey.toBuffer(),
                Buffer.from("whitelist_account"),
                usr1Keypair.publicKey.toBuffer()
            ],
            program.programId
        );

        const [baseTokenAccount] = await anchor.web3.PublicKey.findProgramAddress(
            [
                poolKeyPair.publicKey.toBuffer(),
                Buffer.from("base_token_account"),
                baseTokenMintPubkey.toBuffer()
            ],
            program.programId
        );

        const [quoteTokenAccount] = await anchor.web3.PublicKey.findProgramAddress(
            [
                poolKeyPair.publicKey.toBuffer(),
                Buffer.from("quote_token_account"),
                quoteTokenMintPubkey.toBuffer()
            ],
            program.programId
        );

        let baseTokenUserAccount = await getATAAddress({
            mint: baseTokenMintPubkey,
            owner: fundraiserKeypair.publicKey
        });

        const [nftLockAccount] = await anchor.web3.PublicKey.findProgramAddress(
            [
                poolKeyPair.publicKey.toBuffer(),
                Buffer.from("nft_lock_account"),
                userNFTPubkey.toBuffer()
            ],
            program.programId
        );

        let nftUserAccount = await getATAAddress({
            mint: userNFTPubkey,
            owner: usr1Keypair.publicKey
        })


        const [ticketLockAccount] = await anchor.web3.PublicKey.findProgramAddress(
            [
                poolKeyPair.publicKey.toBuffer(),
                Buffer.from("ticket_lock_account"),
                usr1Keypair.publicKey.toBuffer()
            ],
            program.programId
        );

        let ticketUserAccount = await getATAAddress({
            mint: TICKET_MINT_PUBKEY,
            owner: usr1Keypair.publicKey
        });


        try {

            let whitelistAccountData = await program.account.whitelistAccount.fetch(whitelistAccount);
            printWhitelistAccount(whitelistAccountData);
            let preInstructions = [];

            //locked nft var, unlock ekleyelim
            if (whitelistAccountData.lockedNftMint.toBase58() != web3.PublicKey.default.toBase58()) {
                const unlockNftInstructions = await program.methods.unlockNft().accounts({
                    poolAccount: poolKeyPair.publicKey,
                    whitelistAccount: whitelistAccount,
                    participant: usr1Keypair.publicKey,
                    nftLockAccount: nftLockAccount,
                    nftUserAccount: nftUserAccount,
                    nftMint: userNFTMint,
                    tokenProgram: TOKEN_PROGRAM_ID,
                    systemProgram: anchor.web3.SystemProgram.programId,
                }).signers([usr1Keypair]).instruction()

                preInstructions.push(unlockNftInstructions)
            }

            //locked ticket var, unlock ekleyelim
            if (whitelistAccountData.lockedTicketAmount.toNumber() > 0) {

                const unlockTicketInstructions = await program.methods.unlockTicket().accounts({
                    poolAccount: poolKeyPair.publicKey,
                    whitelistAccount: whitelistAccount,
                    participant: usr1Keypair.publicKey,
                    ticketLockAccount: ticketLockAccount,
                    ticketUserAccount: ticketUserAccount,
                    ticketMint: TICKET_MINT_PUBKEY,
                    tokenProgram: TOKEN_PROGRAM_ID,
                    systemProgram: anchor.web3.SystemProgram.programId,
                }).signers([usr1Keypair]).instruction()

                preInstructions.push(unlockTicketInstructions);
            }


            if (!(await program.provider.connection.getAccountInfo(baseTokenAccount))) {
                preInstructions.push(createATAInstruction({
                    mint: baseTokenMintPubkey,
                    address: baseTokenAccount,
                    owner: usr1Keypair.publicKey,
                    payer: usr1Keypair.publicKey,
                }))
            }

            if (!(await program.provider.connection.getAccountInfo(ticketUserAccount))) {
                preInstructions.push(createATAInstruction({
                    mint: TICKET_MINT_PUBKEY,
                    address: ticketUserAccount,
                    owner: usr1Keypair.publicKey,
                    payer: usr1Keypair.publicKey,
                }))
            }

            if (!(await program.provider.connection.getAccountInfo(nftUserAccount))) {
                preInstructions.push(createATAInstruction({
                    mint: userNFTPubkey,
                    address: nftUserAccount,
                    owner: usr1Keypair.publicKey,
                    payer: usr1Keypair.publicKey,
                }))
            }


            const tx = await program.methods.claim().accounts({
                poolAccount: poolKeyPair.publicKey,
                whitelistAccount: whitelistAccount,
                quoteTokenAccount: quoteTokenAccount,
                baseTokenAccount: baseTokenAccount,
                baseTokenUserAccount: baseTokenUserAccount,
                quoteMint: quoteTokenMintPubkey,
                participant: usr1Keypair.publicKey,
                tokenProgram: TOKEN_PROGRAM_ID,
                systemProgram: anchor.web3.SystemProgram.programId,
            })
                .signers([usr1Keypair])
                .preInstructions(preInstructions)
                .rpc();

            console.log("Unlock & Claim tx: ", tx)

        } catch (e) {
            console.log(e);
            throw e;
        }

    });

    it('claim-again', async () => {

        try {

            const [whitelistAccount] = await anchor.web3.PublicKey.findProgramAddress(
                [
                    poolKeyPair.publicKey.toBuffer(),
                    Buffer.from("whitelist_account"),
                    usr1Keypair.publicKey.toBuffer()
                ],
                program.programId
            );

            const [baseTokenAccount] = await anchor.web3.PublicKey.findProgramAddress(
                [
                    poolKeyPair.publicKey.toBuffer(),
                    Buffer.from("base_token_account"),
                    baseTokenMintPubkey.toBuffer()
                ],
                program.programId
            );

            const [quoteTokenAccount] = await anchor.web3.PublicKey.findProgramAddress(
                [
                    poolKeyPair.publicKey.toBuffer(),
                    Buffer.from("quote_token_account"),
                    quoteTokenMintPubkey.toBuffer()
                ],
                program.programId
            );

            let baseTokenUserAccount = await getATAAddress({
                mint: baseTokenMintPubkey,
                owner: fundraiserKeypair.publicKey
            });


            const tx = await program.methods.claim().accounts({
                poolAccount: poolKeyPair.publicKey,
                whitelistAccount: whitelistAccount,
                quoteTokenAccount: quoteTokenAccount,
                baseTokenAccount: baseTokenAccount,
                baseTokenUserAccount: baseTokenUserAccount,
                quoteMint: quoteTokenMintPubkey,
                participant: usr1Keypair.publicKey,
                tokenProgram: TOKEN_PROGRAM_ID,
                systemProgram: anchor.web3.SystemProgram.programId,
            }).signers([usr1Keypair]).rpc();

            console.log("claim transaction signature", tx);
        } catch (e) {
            console.log(e);
            throw e;
        }

    });

    it('close whitelist account', async () => {

        const [whitelistAccount] = await anchor.web3.PublicKey.findProgramAddress(
            [
                poolKeyPair.publicKey.toBuffer(),
                Buffer.from("whitelist_account"),
                usr1Keypair.publicKey.toBuffer()
            ],
            program.programId
        );

        const [quoteTokenAccount] = await anchor.web3.PublicKey.findProgramAddress(
            [
                poolKeyPair.publicKey.toBuffer(),
                Buffer.from("quote_token_account"),
                quoteTokenMintPubkey.toBuffer()
            ],
            program.programId
        );

        const [ticketLockAccount] = await anchor.web3.PublicKey.findProgramAddress(
            [
                poolKeyPair.publicKey.toBuffer(),
                Buffer.from("ticket_lock_account"),
                usr1Keypair.publicKey.toBuffer()
            ],
            program.programId
        );


        let ticketUserAccount = await getATAAddress({
            mint: TICKET_MINT_PUBKEY,
            owner: usr1Keypair.publicKey
        });

        const [nftLockAccount] = await anchor.web3.PublicKey.findProgramAddress(
            [
                poolKeyPair.publicKey.toBuffer(),
                Buffer.from("nft_lock_account"),
                userNFTPubkey.toBuffer()
            ],
            program.programId
        );

        let nftUserAccount = await getATAAddress({
            mint: userNFTPubkey,
            owner: usr1Keypair.publicKey
        })

        let quoteUserAccount = await getATAAddress({
            mint: quoteTokenMintPubkey,
            owner: usr1Keypair.publicKey
        })

        try {

            const tx = await program.methods.closeWhitelistAccount().accounts({
                poolAccount: poolKeyPair.publicKey,
                whitelistAccount: whitelistAccount,
                authority: (anchor.getProvider() as AnchorProvider).wallet.publicKey,
                participant: usr1Keypair.publicKey,
                ticketLockAccount: ticketLockAccount,
                ticketUserAccount: ticketUserAccount,
                ticketMint: TICKET_MINT_PUBKEY,
                nftLockAccount: nftLockAccount,
                nftUserAccount: nftUserAccount,
                nftMint: userNFTPubkey,
                quoteTokenAccount: quoteTokenAccount,
                quoteMint: quoteTokenMintPubkey,
                quoteUserAccount: quoteUserAccount,
                tokenProgram: TOKEN_PROGRAM_ID,
                systemProgram: anchor.web3.SystemProgram.programId,
                rent: anchor.web3.SYSVAR_RENT_PUBKEY,
            }).rpc();

            console.log("closeWhitelistAccount transaction signature", tx);
        } catch (e) {
            console.log(e);
            throw e;
        }
    });

    it('close pool', async () => {
        return;
        const tx = await program.methods.closePool().accounts({
            poolAccount: poolKeyPair.publicKey,
            authority: (anchor.getProvider() as AnchorProvider).wallet.publicKey,
        }).rpc();

        console.log("ClosePool transaction signature", tx);
    });

});
