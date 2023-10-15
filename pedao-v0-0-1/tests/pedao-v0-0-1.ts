import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { PedaoV001 } from "../target/types/pedao_v0_0_1";

describe("pedao-v0-0-1 ", () => {

    anchor.setProvider(anchor.AnchorProvider.env());
    const program = anchor.workspace.PedaoV001 as Program<PedaoV001>;

    it("create a rate of a street", async() =>{
        const rateKeyPair = anchor.web3.Keypair.generate();

        await program.methods.sendARateStreet('Arthur', 'https://i.pinimg.com/564x/56/e9/4d/56e94d97d290465608654b3d34c5f999.jpg', 'Nice place', -23.589254, -46.690015)
        .accounts({
            myRate: rateKeyPair.publicKey,
            senderOfRate: program.provider.publicKey,
            systemProgram: anchor.web3.SystemProgram.programId
        }).signers([rateKeyPair])
        .rpc();

        const rateStreetAccount = await program.account.rateTheStreet.fetch(rateKeyPair.publicKey);

        console.log(rateStreetAccount);
    })
});
