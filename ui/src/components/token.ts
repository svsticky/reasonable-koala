import {server} from "@/main";
import {ClientInfo} from "@/components/clients";

interface _TokenInfo {
    scope: string,
}

export class TokenInfo {
    scopes: string[];

    constructor(scopes: string[]) {
        this.scopes = scopes;
    }
}

export class Token {

    static async getCurrentInfo(): Promise<TokenInfo> {
        const r = await fetch(`${server}/api/v1/auth/token-info`, {
            headers: {
                'Authorization': `Bearer ${window.localStorage.getItem('access_token')}`
            }
        })

        if(r.status == 401) {
            const client = await ClientInfo.getInternal();
            window.location.href = client.getAuthorizationRedirect();
        }

        const j: _TokenInfo = await r.json();
        return new TokenInfo(
            j.scope.split(" ")
        )
    }

}