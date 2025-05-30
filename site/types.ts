export class CreateWalletRequest {
    keep_copy: boolean;
    identifier: string;
    pin: string;
    recovery_password: string;
    project_id?: string;
}

export class CreateWalletResponse {
    project_id: boolean;
    identifier: string;
    public_key: string;
    created_at: string;
    shard: string
}