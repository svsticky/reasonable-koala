import {server} from "@/main";

interface _User {
    name: string,
    is_admin: boolean,
}

export class User {
    name: string;
    isAdmin: boolean;

    constructor(name: string, isAdmin: boolean) {
        this.name = name;
        this.isAdmin = isAdmin;
    }

    static async getCurrent(): Promise<User> {
        const r = await fetch(`${server}/api/v1/user/info`, {
            headers: {
                'Authorization': `Bearer ${window.localStorage.getItem('access_token')}`
            }
        })

        const j: _User = await r.json();
        return new User(j.name, j.is_admin);
    }
}