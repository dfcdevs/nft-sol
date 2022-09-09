import * as anchor from "@project-serum/anchor";
// ** Comment this to use solpg imported IDL **
import {
    createKeypairFromFile,
    DEPLOYER_KEY_PATH,
    derivePda,
    generateKeypair,
    showAllOrder,
    updatePriceAllOrder,
} from './util';
import { MarketSol } from "../target/types/market_sol";


describe("transfer-nft", async () => {

    const provider = anchor.AnchorProvider.env()
    const wallet = provider.wallet as anchor.Wallet;
    anchor.setProvider(provider);

    const program = anchor.workspace.MarketSol as anchor.Program<MarketSol>;

    var owner_token: anchor.web3.Keypair;
    var receiver: anchor.web3.Keypair;

    before(async function () {
        owner_token = await createKeypairFromFile(__dirname + "/keypairs/buyer.json");
        receiver = await createKeypairFromFile(DEPLOYER_KEY_PATH);
        console.log("=============Done before initialize=============");
    });

    it("transfer", async () => {
        const mint: anchor.web3.PublicKey = new anchor.web3.PublicKey(
            "8CpUJu8unrNyQeQnJJuuMZtRkLce6xq6UPABXfnhCMSg"
        );

        const ownerTokenAddress = await anchor.utils.token.associatedAddress({
            mint: mint,
            owner: owner_token.publicKey
        });
        const receiverTokenAddress = await anchor.utils.token.associatedAddress({
            mint: mint,
            owner: receiver.publicKey,
        });
        console.log(`Owner's Token Address: ${ownerTokenAddress}`);
        console.log(`Receiver's Token Address: ${receiverTokenAddress}`);

        try {
            await program.methods.transfer()
                .accounts({
                    mint: mint,
                    ownerTokenAccount: ownerTokenAddress,
                    ownerAuthority: owner_token.publicKey,
                    receiverTokenAccount: receiverTokenAddress,
                    receiverAuthority: receiver.publicKey,
                })
                .signers([owner_token])
                .rpc();
        } catch (e) {
            console.log(e);
        }
    });
});