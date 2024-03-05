<template>
    <v-container>
        <v-card max-width="900" class="mx-auto">
            <v-card-title>Home</v-card-title>

            <div v-if="loading">
                <v-card-text>
                    <v-progress-circular indeterminate></v-progress-circular>
                </v-card-text>
            </div>
            <div v-else-if="isAuthenticated">
                <v-card-text>
                    <div>Hi, {{ userName }}</div>

                    <!-- By default users will not be authorized as managers -->
                    <!-- This button gives them the option to authorize with that scope -->
                    <v-btn
                        v-if="internalClient && !wilfordManageScope"
                        :href="internalClient!.getAuthorizationRedirect(true)">
                        Authorize as admin
                    </v-btn>

                    <!-- Manager pages -->
                    <div v-if="wilfordManageScope">
                        <v-card>
                            <v-card-title>Managment</v-card-title>
                            <v-list>
                                <v-list-item v-for="item in adminLinks" :key="item.to">
                                    <v-btn
                                        :flat="true"
                                        prepend-icon="mdi-link"
                                        :to="item.to">
                                        {{ item.name }}
                                    </v-btn>
                                </v-list-item>
                            </v-list>
                        </v-card>
                    </div>
                </v-card-text>
            </div>
        </v-card>
    </v-container>
</template>

<script setup lang="ts">

import {onMounted, Ref, ref} from "vue";
import {User} from "@/components/user";
import {ClientInfo} from "@/components/clients";
import {Token} from "@/components/token";

let loading = ref(true);
let internalClient: Ref<ClientInfo | null> = ref(null);

let isAuthenticated = ref(false);
let userName: Ref<string | null> = ref(null);
let wilfordManageScope = ref(false);

const adminLinks: { name: string, to: string}[] = [
    {
        name: "Manage users",
        to: "/manager/users"
    },
    {
        name: "Manage CAT tokens",
        to: "/manager/cat"
    },
    {
        name: "Manage OAuth2 clients",
        to: "/manager/clients"
    }
];

onMounted(async () => {
    internalClient.value = await ClientInfo.getInternal();

    if(!window.localStorage.getItem('access_token')) {
        isAuthenticated.value = true;
        loading.value = false;

        window.location.href = internalClient.value!.getAuthorizationRedirect();

        return;
    }

    await getUserInfo();
    await getTokenInfo();
})

async function getTokenInfo() {
    const currentToken = await Token.getCurrentInfo();
    wilfordManageScope.value = currentToken.scopes.includes('koala.manage');
}

async function getUserInfo() {
    const currentUser = await User.getCurrent();
    userName.value = currentUser.name;
    isAuthenticated.value = true;
    loading.value = false;
}

</script>