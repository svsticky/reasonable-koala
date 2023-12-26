<template>
    <v-container>
        <v-banner :text="errorBanner" v-if="errorBanner">
            <template v-slot:actions>
                <v-btn
                    v-if="internalClient"
                    :href="internalClient!.getAuthorizationRedirect()">
                    Try again
                </v-btn>
            </template>
        </v-banner>

        <v-card v-if="loading">
            <v-card-title>Almost there..</v-card-title>
            <v-card-subtitle>Finishing up logging in</v-card-subtitle>
            <v-card-text>
                <v-progress-circular indeterminate></v-progress-circular>
            </v-card-text>
        </v-card>

        <v-card v-else>
            <v-card-title>Logged in</v-card-title>
            <v-card-subtitle v-if="!errorBanner">Welcome to Wilford.</v-card-subtitle>

            <v-card-actions>

            </v-card-actions>
        </v-card>
    </v-container>
</template>

<script setup lang="ts">

import {onMounted, Ref, ref} from "vue";
import {useRoute, useRouter} from "vue-router";
import {server} from "@/main";
import {ClientInfo} from "@/components/clients";

const router = useRouter();
const route = useRoute();

let errorBanner: Ref<string | undefined> = ref(undefined);
let loading = ref(true);
let internalClient: Ref<ClientInfo | null> = ref(null);

onMounted(async () => {
    await router.isReady();
    await exchangeCodeForToken();
})

async function exchangeCodeForToken() {
    internalClient.value = await ClientInfo.getInternal();
    const r = await fetch(`${server}/api/oauth/token?grant_type=authorization_code&code=${route.query['code']}&redirect_uri=${internalClient!.value?.redirectUri}&client_id=${internalClient!.value?.clientId}&client_secret=${internalClient!.value.clientSecret}`, {
        method: 'POST'
    });

    interface Response {
        access_token?: string,
        error?: string,
    }

    const j: Response = await r.json();
    loading.value = false;

    if(j.error) {
        errorBanner.value = "Could not log in."
    } else {
        window.localStorage.setItem('access_token', j.access_token!);
        await router.push('/');
    }
}

</script>