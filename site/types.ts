export class CreateWalletRequest {
	keep_copy: boolean;
	identifier: string;
	pin: string;
	recovery_password: string;
	project_id?: string;
}

export class GetWalletResponse {
	project_id: string;
	identifier: string;
	seed_hash: string;
	created_at: string;
	shards: string[];
}


