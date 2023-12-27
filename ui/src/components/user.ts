import {server} from "@/main";
import {ClientInfo} from "@/components/clients";

interface _User {
    name: string,
    espo_user_id: string,
    is_admin: boolean,
}

export class User {
    name: string;
    espoUserId: string;
    isAdmin: boolean;

    constructor(name: string, espoUserId: string, isAdmin: boolean) {
        this.name = name;
        this.espoUserId = espoUserId;
        this.isAdmin = isAdmin;
    }

    static async getCurrent(): Promise<User> {
        const r = await fetch(`${server}/api/v1/user/info`, {
            headers: {
                'Authorization': `Bearer ${window.localStorage.getItem('access_token')}`
            }
        })

        if(r.status == 401) {
            const client = await ClientInfo.getInternal();
            window.location.href = client.getAuthorizationRedirect();
        }

        const j: _User = await r.json();
        return new User(j.name, j.espo_user_id, j.is_admin);
    }

    static async list(): Promise<User[]> {
        const r = await fetch(`${server}/api/v1/user/list`, {
            headers: {
                'Authorization': `Bearer ${window.localStorage.getItem('access_token')}`
            }
        })

        interface Response {
            users: _User[]
        }

        const j: Response = await r.json();
        return j.users.map(u => new User(u.name, u.espo_user_id, u.is_admin))
    }

    async listPermittedScopes(): Promise<string[]> {
        const r = await fetch(`${server}/api/v1/user/permitted-scopes/list?user=${this.espoUserId}`, {
            headers: {
                'Authorization': `Bearer ${window.localStorage.getItem('access_token')}`
            }
        })

        interface Response {
            scopes: string[]
        }

        const j: Response = await r.json();
        return j.scopes;
    }

    async deletePermittedScope(scope: string) {
        const r = await fetch(`${server}/api/v1/user/permitted-scopes/remove`, {
            method: 'DELETE',
            body: JSON.stringify({
                from: this.espoUserId,
                scope: scope,
            }),
            headers: {
                'Content-Type': 'application/json',
                'Authorization': `Bearer ${window.localStorage.getItem('access_token')}`
            }
        })
    }

    async addPermittedScope(scope: string) {
        await fetch(`${server}/api/v1/user/permitted-scopes/add`, {
            method: 'POST',
            body: JSON.stringify({
                to: this.espoUserId,
                scope: scope,
            }),
            headers: {
                'Content-Type': 'application/json',
                'Authorization': `Bearer ${window.localStorage.getItem('access_token')}`
            }
        })
    }
}