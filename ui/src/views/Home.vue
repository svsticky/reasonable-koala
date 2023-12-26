<template>
    <v-container>
        <v-card>
            <v-card-title>Home</v-card-title>

            <div v-if="loading">
                <v-card-text>
                    <v-progress-circular indeterminate></v-progress-circular>
                </v-card-text>
            </div>
            <div v-else-if="isAuthenticated">
                <v-card-text>
                    <div>Hi, {{ userName }}</div>
                </v-card-text>
            </div>
        </v-card>
    </v-container>
</template>

<script setup lang="ts">

import {onMounted, Ref, ref} from "vue";
import {User} from "@/components/user";
import {ClientInfo} from "@/components/clients";

let loading = ref(true);
let isAuthenticated = ref(false);
let userName: Ref<string | null> = ref(null);

onMounted(async () => {
    if(!window.localStorage.getItem('access_token')) {
        isAuthenticated.value = true;
        loading.value = false;

        const internalClient = await ClientInfo.getInternal();
        window.location.href = internalClient.getAuthorizationRedirect();

        return;
    }

    await getTokenInfo();
})

async function getTokenInfo() {
    const currentUser = await User.getCurrent();
    userName.value = currentUser.name;
    isAuthenticated.value = true;
    loading.value = false;
}

</script>