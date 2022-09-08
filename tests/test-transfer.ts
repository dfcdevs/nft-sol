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

    var deployer: anchor.web3.Keypair;
    var receiver: anchor.web3.Keypair;

    before(async function () {
        receiver = await createKeypairFromFile(__dirname + "/keypairs/receiver.json");
        deployer = await createKeypairFromFile(DEPLOYER_KEY_PATH);
        console.log("=============Done before initialize=============");
    });

    it("transfer", async () => {
        const mint: anchor.web3.PublicKey = new anchor.web3.PublicKey(
            "euwDXm4f1NoZnTpqHK9HsBBY8iXv8HjnmgVsmMA3FEk"
        );

        const ownerTokenAddress = await anchor.utils.token.associatedAddress({
            mint: mint,
            owner: deployer.publicKey
        });
        const receiverTokenAddress = await anchor.utils.token.associatedAddress({
            mint: mint,
            owner: receiver.publicKey,
        });
        console.log(`Owner's Token Address: ${ownerTokenAddress}`);
        console.log(`Receiver's Token Address: ${receiverTokenAddress}`);

        try {
            await program.methods.transfer(
            )
                .accounts({
                    mint: mint,
                    ownerTokenAccount: ownerTokenAddress,
                    ownerAuthority: wallet.publicKey,
                    vaultTokenAccount: receiverTokenAddress,
                    vaultAuthority: receiver.publicKey,
                })
                .signers([receiver])
                .rpc();
        } catch(e) {
            console.log(e);
        }
    });
});