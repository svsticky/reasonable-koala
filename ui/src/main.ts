/**
 * main.ts
 *
 * Bootstraps Vuetify and other plugins then mounts the App`
 */

// Plugins
import { registerPlugins } from '@/plugins'

export const server = 
window.location.host.includes("localhost") ? "http://localhost:2521"    // Dev, in docker
: window.location.host.includes("127.0.0.1") ? "http://localhost:8080"  // Dev, not docker
: "/wilford";                                                                   // Production

// Components
import App from './App.vue'

// Composables
import { createApp } from 'vue'

const app = createApp(App)

// Vuetify missing type
export type RuleFn = (v: string) => string | boolean;
export type DataTableHeader = { title: string, value: string }
/**
 * The item you get when you click a row
 * E.g: @click:row="(_: any, v: RowItem<Foo>) => console.log(v.item))"
 * Will print the T.
 */
export type RowItem<T> = { item: T }


registerPlugins(app)

app.mount('#app')
