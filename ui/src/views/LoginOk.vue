<template>
    <v-container>
        <v-card>
            <v-card-title>Almost there..</v-card-title>
            <v-card-subtitle>Finishing up logging in</v-card-subtitle>
            <v-card-text>
                <v-progress-circular indeterminate></v-progress-circular>
            </v-card-text>
        </v-card>
    </v-container>
</template>

<script setup lang="ts">

import {onMounted, ref} from "vue";
import {useRouter} from "vue-router";
import {server} from "@/main";

const router = useRouter();

let loading = ref(true);

onMounted(async () => {
    await router.isReady();
    await exchangeCodeForToken();
})

async function exchangeCodeForToken() {
    const r = await fetch(`${server}/api/oauth/token?gra`)
}

</script>