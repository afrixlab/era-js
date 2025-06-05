import { sr25519 } from "@polkadot-labs/hdkd-helpers"
import { getPolkadotSigner } from "@polkadot-api/signer";


export class Signer {
	private privateKey: Uint8Array;

	constructor(privateKey: Uint8Array) {
		this.privateKey = privateKey;
	}

	getPolkadotSigner() {
		const signer = getPolkadotSigner(
			sr25519.getPublicKey(this.privateKey),
			"Sr25519",
			(input) => sr25519.sign(input, this.privateKey)
		);
        return signer;
	}
}