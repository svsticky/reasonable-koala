import {server} from "@/main";
import {ClientInfo} from "@/components/clients";

interface _User {
    name: string,
    email: string,
    user_id: string,
    is_admin: boolean,
}

export class User {
    name: string;
    userId: string;
    email: string;
    isAdmin: boolean;

    constructor(name: string, userId: string, email: string, isAdmin: boolean) {
        this.name = name;
        this.userId = userId;
        this.email = email;
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
        return new User(j.name, j.user_id, j.email, j.is_admin);
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
        return j.users.map(u => new User(u.name, u.user_id, u.email, u.is_admin))
    }

    async listPermittedScopes(): Promise<string[]> {
        const r = await fetch(`${server}/api/v1/user/permitted-scopes/list?user=${this.userId}`, {
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
                from: this.userId,
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
                to: this.userId,
                scope: scope,
            }),
            headers: {
                'Content-Type': 'application/json',
                'Authorization': `Bearer ${window.localStorage.getItem('access_token')}`
            }
        })
    }
}