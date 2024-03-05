<template>
    <v-container>
        <v-card max-width="600" class="mx-auto">
            <v-card-title>Register</v-card-title>
            <v-card-subtitle>Register a new Koala account</v-card-subtitle>

            <v-card-text>
                <v-img
                    height="200"
                    src="https://public.svsticky.nl/logos/logo_outline_kleur.png"
                    contain
                ></v-img>

                <v-form v-model="valid">
                    <v-text-field
                        v-model="name"
                        label="Name"
                        :rules="rules.name"
                    ></v-text-field>

                    <v-text-field
                        v-model="email"
                        label="Email"
                        :rules="rules.email"
                    ></v-text-field>

                    <v-text-field
                        v-model="password"
                        label="Password"
                        type="password"
                        :rules="rules.password"
                    ></v-text-field>

                    <v-text-field
                        v-model="password1"
                        label="Repeat password"
                        type="password"
                        :rules="rules.password1"
                    ></v-text-field>
                </v-form>
            </v-card-text>
            <v-card-actions>
                <v-btn
                    @click="navigateToLogin"
                    :loadin="loading.navigateToLogin"
                    :disabled="loading.navigateToLogin">
                    Login instead
                </v-btn>
                <v-spacer></v-spacer>
                <v-btn
                    @click="register"
                    :loading="loading.register"
                    :disabled="!valid || loading.register">
                    Register
                </v-btn>
            </v-card-actions>
        </v-card>
    </v-container>
</template>

<script lang="ts">
import {defineComponent} from 'vue'
import {RuleFn} from "@/main";
import {ClientInfo} from "@/components/clients";

interface Data {
    loading: {
        register: boolean,
        navigateToLogin: boolean,
    },
    name: string,
    email: string,
    password: string,
    password1: string,
    valid: boolean,
    rules: {
        name: RuleFn[],
        email: RuleFn[],
        password: RuleFn[],
        password1: RuleFn[],
    }
}

export default defineComponent({
    data(): Data {
        return {
            loading: {
                register: false,
                navigateToLogin: false,
            },
            name: "",
            email: "",
            password: "",
            password1: "",
            valid: false,
            rules: {
                name: [
                    v => !!v || "Required"
                ],
                email: [
                    v => !!v || "Required",
                    v => /[^@ \t\r\n]+@[^@ \t\r\n]+\.[^@ \t\r\n]+/.test(v) || "Invalid E-mail address"
                ],
                password: [
                    v => !!v || "Required",
                ],
                password1: [
                    v => !!v || "Required",
                    v => v == this.password || "Password mismatch"
                ]
            }
        }
    },
    methods: {
        async navigateToLogin() {
            const client = await ClientInfo.getInternal();
            window.location.href = client.getAuthorizationRedirect();
        },
        async register() {

        }
    }
})
</script>