<template>
    <v-app-bar flat>
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
    </v-app-bar>
</template>

<style scoped lang="scss">
@import '@/styles/appbar';
</style>

<script lang="ts">

import {defineComponent} from "vue";

export default defineComponent({})
</script>
