<template>
    <v-container>
        <v-card>
            <v-card-title>Authorize</v-card-title>
            <v-card-subtitle>Grant authorization</v-card-subtitle>

            <div v-if="loading">
                <v-card-text>
                    <v-progress-circular indeterminate></v-progress-circular>
                </v-card-text>
            </div>
            <div v-else>
                <v-card-text>
                    Grant '{{ clientName }}' access to your account?
                    <div v-if="scopes && scopes.length > 0">
                        Scopes:
                        <v-list>
                            <v-list-item v-for="scope in scopes">
                                {{ scope }}
                            </v-list-item>
                        </v-list>

                    </div>
                </v-card-text>

                <v-card-actions>
                    <v-btn
                        @click="denyAuth">
                        Deny
                    </v-btn>
                    <v-spacer></v-spacer>
                    <v-btn
                        @click="allowAuth">
                        Allow
                    </v-btn>
                </v-card-actions>
            </div>
        </v-card>
    </v-container>
</template>

<script setup lang="ts">

import {ref, onMounted, Ref} from "vue";
import {server} from "@/main";
import {useRoute, useRouter} from "vue-router";

const route = useRoute();
const router = useRouter();

let loading = ref(true);
let clientName: Ref<string | null> = ref(null);
let scopes: Ref<string[] | null> = ref([]);

onMounted(async () => {
    await router.isReady();
    await loadAuthorizationInfo();
});

async function allowAuth() {
    window.location.href = `${server}/api/v1/auth/authorize?authorization=${route.query['authorization']}&grant=true`
}

async function denyAuth() {
    window.location.href = `${server}/api/v1/auth/authorize?authorization=${route.query['authorization']}&grant=false`
}

async function loadAuthorizationInfo() {
    const r = await fetch(`${server}/api/v1/auth/authorization-info?authorization=${route.query['authorization']}`);
    switch(r.status) {
        case 200:
            interface Response {
                client_name: string,
                scopes?: string,
            }

            const json: Response = await r.json();
            clientName.value = json.client_name;
            scopes.value = json.scopes?.split(" ") ?? [];
            loading.value = false;
            console.log(loading.value)
            break;
        default:
            break;
    }
}

</script>