import * as anchor from "@project-serum/anchor";
// ** Comment this to use solpg imported IDL **
import {
    createKeypairFromFile,
    derivePda,
    generateKeypair,
    showAllOrder,
    updatePriceAllOrder,
} from './util';
import { MarketSol } from "../target/types/market_sol";


describe("ordering-nft", async () => {

    const provider = anchor.AnchorProvider.env()
    const wallet = provider.wallet as anchor.Wallet;
    anchor.setProvider(provider);
    const program = anchor.workspace.MarketSol as anchor.Program<MarketSol>;

    var list_order_account: anchor.web3.Keypair;
    var pda;
    const mint: anchor.web3.PublicKey = new anchor.web3.PublicKey(
        "49swiGRKuZnQvXTdhnGhQae5PYzmzaQKVhMxxJn3M6JF"
    );

    var deployer: anchor.web3.Keypair;
    var vault: anchor.web3.Keypair;

    before(async function () {
        // list_order_account = await createKeypairFromFile(__dirname + "/keypairs/vault.local.json");
        // order_account = await generateKeypair(provider);

        // console.log(list_order_account.publicKey.toString());
        // console.log(order_account.publicKey.toString());

        list_order_account = await createKeypairFromFile(__dirname + "/keypairs/vault.json");
        deployer = await createKeypairFromFile("/Users/menduong/.config/solana/id.json");
        vault = list_order_account;
        console.log(list_order_account.publicKey.toString());
        console.log("==========Done before initialize=============");
    });

    it('setup', async () => {
        await program.methods.setupPlatform()
            .accounts({
                listOrderAccount: list_order_account.publicKey,
                userOwner: list_order_account.publicKey,
                systemProgram: anchor.web3.SystemProgram.programId,
            })
            .signers([list_order_account])
            .rpc();

        let list_order = await (await program.account.listOrder.fetch(list_order_account.publicKey)).data;
        console.log(list_order);
    });

    it('clear', async () => {
        await program.methods.clearData()
            .accounts({
                listOrderAccount: list_order_account.publicKey,
                userOwner: list_order_account.publicKey,
                systemProgram: anchor.web3.SystemProgram.programId,
            })
            .signers([list_order_account])
            .rpc();

        let list_order = await (await program.account.listOrder.fetch(list_order_account.publicKey)).data;
        console.log(list_order);
    });

    it("listing", async () => {
        const saleAmount = 1 * anchor.web3.LAMPORTS_PER_SOL;
        console.log(`Vault public key: ${vault.publicKey}`);

        // Derive the associated token account address for owner & buyer

        const ownerTokenAddress = await anchor.utils.token.associatedAddress({
            mint: mint,
            owner: wallet.publicKey
        });
        const vaultTokenAddress = await anchor.utils.token.associatedAddress({
            mint: mint,
            owner: vault.publicKey,
        });
        console.log(`Request to sell NFT: ${mint} for ${saleAmount} lamports`);
        console.log(`Owner's Token Address: ${ownerTokenAddress}`);
        console.log(`Vault's Token Address: ${vaultTokenAddress}`);

        // const fake_mint: anchor.web3.PublicKey = new anchor.web3.PublicKey(
        //     "2VfJVR3wpX65A3fdMnhmyu63Mv2wX1QZRVfRCCiYk3fJ"
        // );
        pda = await derivePda(mint, program);
        // console.log(`Checking if account ${(pda)} exists for token_id: ${fake_mint}`);
        // Transact with the "sell" function in our on-chain program

        await program.methods.createOrder(
            new anchor.BN(saleAmount)
        )
            .accounts({
                orderAccount: pda,
                listOrderAccount: list_order_account.publicKey,
                mint: mint,
                ownerTokenAccount: ownerTokenAddress,
                ownerAuthority: wallet.publicKey,
                vaultTokenAccount: vaultTokenAddress,
                vaultAuthority: list_order_account.publicKey,
            })
            .signers([vault])
            .rpc();
    });

    it("fetch all list order", async () => {
        await showAllOrder(list_order_account.publicKey, program);
    });

    it("update price all order", async () => {
        updatePriceAllOrder(deployer, program, list_order_account.publicKey);
    });

    it("associate ", async () => {
        const mock_mint: anchor.web3.PublicKey = new anchor.web3.PublicKey(
            "6LFygBCud2wJmNqVobJ5g5oJM3UUbD4vNByTXKsy4BK1"
        );
        const buyer2: anchor.web3.Keypair = await createKeypairFromFile(__dirname + "/keypairs/buyer2.json");
        console.log(`Buyer public key: ${buyer2.publicKey}`);

        // Derive the associated token account address for owner & buyer

        const tokenAddress = await anchor.utils.token.associatedAddress({
            mint: mock_mint,
            owner: buyer2.publicKey
        });

        console.log("tokenAccountAddress: ", tokenAddress.toString());
    });
});