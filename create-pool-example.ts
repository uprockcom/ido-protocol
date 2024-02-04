import * as anchor from '@project-serum/anchor';
import {IDL} from './target/types/ido_protocol';
import NodeWallet from "@project-serum/anchor/dist/cjs/nodewallet";
import {Connection} from "@solana/web3.js";
import {TOKEN_PROGRAM_ID} from "@solana/spl-token";

function startsEndsToJson(se) {
    return {
        startsAt: new Date(se.startsAt.toNumber() * 1000).toUTCString(),
        endsAt: new Date(se.endsAt.toNumber() * 1000).toUTCString()
    }
}

const PROGRAM_ID = "ido5zRdfphtyeov8grJqBszfWquvTCAvRzGycSfPXuX";
const CLUSTER = "https://api.mainnet-beta.solana.com"
const TICKET_MINT_PUBKEY = new anchor.web3.PublicKey("TicpC2VhBZbknfc4RhYkdPty6SE4HjBozZS9umAMX1d");

const DISTRIBUTION_TYPE_STANDARD = 0;
const DISTRIBUTION_TYPE_VESTED = 1;
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
    console.log("idoPoolAccount.distributionVestingPeriod (seconds) ", idoPoolAccount.distributionVestingPeriod.toNumber())

    for (let i = 0; i < idoPoolAccount.distributionVesting.length; i++) {
        console.log("idoPoolAccount.distributionVesting[" + i + "]", idoPoolAccount.distributionVesting[i]);
    }

    console.log("idoPoolAccount.ticketMaxAllocationRate (0-100)  ", idoPoolAccount.ticketMaxAllocationRate)
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


let opts = anchor.AnchorProvider.defaultOptions();
const connection = new Connection(
    CLUSTER,
    opts.preflightCommitment
);

const admin = anchor.web3.Keypair.fromSecretKey("devKH8tdPeByT13FWMVcX48B43v3wB1Qsa2xfGvpttm.json")

let mainnetProvider = new anchor.AnchorProvider(connection, new NodeWallet(admin), opts);
anchor.setProvider(mainnetProvider);
const program = new anchor.Program(IDL, PROGRAM_ID, mainnetProvider);
const poolKeyPair = anchor.web3.Keypair.fromSecretKey("poo4b5Lsyg5fMdrbmXkf6apxTNKQrAx4booXkbGVNNW.json")
const fundraiserKeypair = anchor.web3.Keypair.fromSecretKey("funJG6uooFXjJpALB6ExhnBefsPFJkamzy6Gfv7zN98.json")


console.log("poolId", poolKeyPair.publicKey.toBase58())
console.log("fundraiser", fundraiserKeypair.publicKey.toBase58())


describe('ido-protocol', () => {

    let baseTokenMintAddress = 'tokBWvxCaa1AJnVUUbgeNHeVDJYbu1DpYXS9yTuvmjY';
    let baseTokenDecimals = 8;
    let baseTokenMintPubkey = new anchor.web3.PublicKey(baseTokenMintAddress);

    let quoteTokenMintAddress = 'usdA7bUXh1kNAwhCmabJf7QmWsaTk4Mymk26aEsjAeB';
    let quoteTokenDecimals = 6;
    let quoteTokenMintPubkey = new anchor.web3.PublicKey(quoteTokenMintAddress);

    // const startTime: number = Math.floor((new Date().getTime()) / 1000.0)
    let price = 0.7;


    it('create pool', async () => {

        //     let poolDataInput = {
        //             name: Buffer.from("Secretum"),
        //             distributionType: DISTRIBUTION_TYPE_STANDARD,
        //             fundraiser: fundraiserKeypair.publicKey,
        //             phaseWhitelist: {
        //                 startsAt: new anchor.BN(startTime),
        //                 endsAt: new anchor.BN(startTime + (++stage)),
        //             },
        //             phaseSaleNft: {
        //                 startsAt: new anchor.BN(startTime + (++stage)),
        //                 endsAt: new anchor.BN(startTime + (++stage)),
        //             },
        //             phaseSaleTicket: {
        //                 startsAt: new anchor.BN(startTime + (++stage)),
        //                 endsAt: new anchor.BN(startTime + (++stage)),
        //             },
        //             phaseDistribution: {
        //                 startsAt: new anchor.BN(startTime + (++stage)),
        //                 endsAt: new anchor.BN(startTime + (++stage)),
        //             },
        //             distributionVestingPeriod: new anchor.BN(60), //1 minute per period.
        //             distributionVesting: Buffer.from([25, 10, 10, 10, 10, 10, 10,10, 5]),
        //
        //             walletMinCap: new anchor.BN(Math.pow(10, quoteTokenDecimals) * 100), // 100 quote min
        //             walletMaxCap: new anchor.BN(Math.pow(10, quoteTokenDecimals) * 1000), // 1000 quote max
        //
        //             walletMinTicket: new anchor.BN(Math.pow(10, baseTokenDecimals) * 100), // 100k ticket min
        //             walletMaxTicket: new anchor.BN(Math.pow(10, baseTokenDecimals) * 500), // 900k ticket max
        //             ticketMaxAllocationRate: 100, //no more than 100% allocation for ticket holders
        //
        //             poolHardCap: new anchor.BN(Math.pow(10, quoteTokenDecimals) * 200000), // 200_000 quote max
        //             rate: {
        //                 base: new anchor.BN(Math.pow(10, baseTokenDecimals)),
        //                 quote: new anchor.BN(Math.pow(10, quoteTokenDecimals) * price)
        //             },
        //
        //             allowedCreators: [
        //                 {
        //                     address: new web3.PublicKey("ENRhYNoo6aYxejcPE6ZW3rGkqnYyAdG3VZtBDU91SkE4"),
        //                     verified: true,
        //                     share: 0,
        //                     allocation: 50
        //                 },
        //                 {
        //                     address: new web3.PublicKey("vy9eHdWjrFem1R7Hz7MF56x1Yz111rAteUPbPmrNX4e"),
        //                     verified: true,
        //                     share: 0,
        //                     allocation: 90
        //                 },
        //                 {
        //                     address: new web3.PublicKey("Dk2hFRw1nR1vh8KGtazLps2ztfXbR89e2TxQRK8YJkgA"),
        //                     verified: true,
        //                     share: 0,
        //                     allocation: 100
        //                 },
        //             ]
        //         }

        let poolDataInput = {
            name: Buffer.from("TokenSale"),
            distributionType: DISTRIBUTION_TYPE_STANDARD,
            fundraiser: fundraiserKeypair.publicKey,
            phaseWhitelist: {
                startsAt: new anchor.BN(1650229200), // 4/17/2022, 09:00:00 PM UTC
                endsAt: new anchor.BN(1650618000), // 4/22/2022, 09:00:00 AM UTC
            },
            phaseSaleNft: {
                startsAt: new anchor.BN(1650639600), // 4/22/2022, 03:00:00 PM UTC
                endsAt: new anchor.BN(1650704399), // 4/23/2022, 08:59:59 AM UTC
            },
            phaseSaleTicket: {
                startsAt: new anchor.BN(1650704400), // 4/23/2022, 09:00:00 AM UTC
                endsAt: new anchor.BN(1650790800), //4/24/2022, 09:00:00 AM UTC
            },
            phaseDistribution: {
                startsAt: new anchor.BN(1650898800), // 4/25/2022, 03:00:00 PM UTC
                endsAt: new anchor.BN(1653469200), // 5/25/2022, 03:00:00 PM UTC
            },
            distributionVestingPeriod: new anchor.BN(86400 * 30), //every 30 days
            distributionVesting: Buffer.from([25, 10, 10, 10, 10, 10, 10,10, 5]), //25% first the first distro

            walletMinCap: new anchor.BN(Math.pow(10, quoteTokenDecimals) * 50), // 50 usc quote min
            walletMaxCap: new anchor.BN(Math.pow(10, quoteTokenDecimals) * 2_000), // 2000 usdc quote max

            walletMinTicket: new anchor.BN(Math.pow(10, baseTokenDecimals) * 50_000), // 50k ticket min
            walletMaxTicket: new anchor.BN(Math.pow(10, baseTokenDecimals) * 2_000_000), // 2k ticket max
            ticketMaxAllocationRate: 100, //no more than 100% allocation rate for ticket holders

            poolHardCap: new anchor.BN(Math.pow(10, quoteTokenDecimals) * 100_000), // 100_000 quote max
            rate: {
                base: new anchor.BN(Math.pow(10, baseTokenDecimals)),
                quote: new anchor.BN(Math.pow(10, quoteTokenDecimals) * price)
            },

            allowedCreators: [
                //GAC 1
                {
                    address: new anchor.web3.PublicKey("BwuSuemEWHBdUNcCiQ8m9Nd595mnYD3nCfTkNw6DQvmN"),
                    verified: true,
                    share: 0,
                    allocation: 100
                },
                // GAC 2
                {
                    address: new anchor.web3.PublicKey("9boz19yS9wWoEJDmrYYsmYgezofyoREaL5u88kZu8Sgx"),
                    verified: true,
                    share: 0,
                    allocation: 100
                },
                //IDO Batch
                {
                    address: new anchor.web3.PublicKey("d1ZgnTnc6BftjaBBE6mLbPtGxrjC7qF4Y1FXyLVNMrk"),
                    verified: true,
                    share: 0,
                    allocation: 20
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
                authority: admin.publicKey,
                tokenProgram: TOKEN_PROGRAM_ID,
                systemProgram: anchor.web3.SystemProgram.programId,
                rent: anchor.web3.SYSVAR_RENT_PUBKEY,
            },
            signers: [poolKeyPair],
        };


            const tx = await program.methods.createPool(poolDataInput as never)
                .accounts(createPoolContext.accounts)
                .signers(createPoolContext.signers).rpc();


        let idoPoolAccount = await program.account.idoPoolAccount.fetch(poolKeyPair.publicKey);
        printIdoPoolAccount(idoPoolAccount);

        console.log("CreatePool transaction signature", tx);
    });
});




