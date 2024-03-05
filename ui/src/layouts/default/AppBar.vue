<template>
    <v-app-bar elevation="1">
        <v-img
            class="mx-2 mr-0"
            contain
            max-height="48"
            max-width="48"
            src="https://public.svsticky.nl/logos/hoofd_outline_kleur.png"
        ></v-img>
        <v-app-bar-title class="page-title">
            <a
                target="_blank"
                class="nostyle"
                href="https://github.com/svsticky/reasonable-koala">
                Koala
            </a>
        </v-app-bar-title>

        <v-spacer></v-spacer>

        <v-tooltip text="Sign out">
            <template v-slot:activator="{ props }">
                <v-btn
                    v-bind="props"
                    density="compact"
                    :slim="true"
                    @click="signout">
                    <v-icon>mdi-exit-to-app</v-icon>
                </v-btn>
            </template>
        </v-tooltip>
    </v-app-bar>
</template>

<style scoped lang="scss">
@import '@/styles/appbar';
</style>

<script lang="ts">

import {defineComponent} from "vue";

export default defineComponent({
    methods: {
        signout() {
            window.localStorage.clear();
            if(this.$route.fullPath != "/") {
                this.$router.replace("/");
                console.log("a");
            } else {
                window.location.reload();
            }
        }
    }
})
</script>
