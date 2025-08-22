const {Connection, LAMPORTS_PER_SOL, clusterApiUrl, PublicKey} = require("@solana/web3.js");

const conn = new Connection(clusterApiUrl("devnet"));

async function aridrop(publicKey, amount) {
    const airdropSignature = await conn.requestAirdrop(new PublicKey(publicKey), amount);
    await conn.confirmTransaction({signature: airdropSignature})
}

airdrop("", LAMPORTS_PER_SOL).then(signature => {
    console.log("Airdrop Signature:", signature);
})
