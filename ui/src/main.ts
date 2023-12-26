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
: "";                                                                   // Production

// Components
import App from './App.vue'

// Composables
import { createApp } from 'vue'

const app = createApp(App)

registerPlugins(app)

app.mount('#app')
