import * as anchor from "@project-serum/anchor";
import fs from 'mz/fs';

export const DEPLOYER_KEY_PATH = "/Users/menduong/.config/solana/id.json";

export async function createKeypairFromFile(
    filePath: string,
): Promise<anchor.web3.Keypair> {
    const secretKeyString = await fs.readFile(filePath, { encoding: 'utf8' });
    const secretKey = Uint8Array.from(JSON.parse(secretKeyString));
    return anchor.web3.Keypair.fromSecretKey(secretKey);
}

export async function generateKeypair(provider) {
    let keypair = anchor.web3.Keypair.generate();
    await provider.connection.requestAirdrop(
        keypair.publicKey,
        2 * anchor.web3.LAMPORTS_PER_SOL
    );
    await new Promise(resolve => setTimeout(resolve, 5 * 1000)); // Sleep 3s
    return keypair;
}

export async function derivePda(token_id: anchor.web3.PublicKey, program) {
    let [pda, _] = await anchor.web3.PublicKey.findProgramAddress(
        [
            token_id.toBuffer(),
            Buffer.from("_"),
        ],
        program.programId
    );
    return pda;
}

export async function updatePriceAllOrder(
    owner: anchor.web3.Keypair,
    program,
    pubkey_list_order_account: anchor.web3.PublicKey,
) {
    const priceAdded = 1.1 * anchor.web3.LAMPORTS_PER_SOL;
    let listOrder = await (await program.account.listOrder.fetch(pubkey_list_order_account)).data;
    for (var i = 0; i < listOrder.length; i++) {
        let nft_id = listOrder[i];
        let pda = await derivePda(nft_id, program);
        let data = await program.account.orderDetail.fetch(pda);
        console.log(`Current token_id: ${data.tokenId}   price: ${data.price}  seller: ${data.seller}`);
        // update price +0.1 sol only testing
        const newPrice = await getRandomArbitrary(2, 1) * anchor.web3.LAMPORTS_PER_SOL;
        console.log("newPrice: ", newPrice);
        await program.methods.updateOrderDetail(
            data.tokenId,
            new anchor.BN(newPrice)
        )
            .accounts({
                orderAccount: pda,
                wallet: owner.publicKey,
            })
            .signers([owner])
            .rpc();
    }

    // show all order after udpate price
    await showAllOrder(pubkey_list_order_account, program);
}

export async function showAllOrder(pubKey, program) {
    console.log("Call showAllOrder");
    let listOrder = await (await program.account.listOrder.fetch(pubKey)).data;
    console.log("Total: ", listOrder.length);
    for (var i = 0; i < listOrder.length; i++) {
        let nft_id = listOrder[i];
        let pda = await derivePda(nft_id, program);
        let data = await program.account.orderDetail.fetch(pda);
        // console.log(pda);
        console.log(`Token_id: ${data.tokenId}   price: ${data.price}  seller: ${data.seller}`);
    }
}

export async function getRandomArbitrary(min, max) {
    return Math.random() * (max - min) + min;
}