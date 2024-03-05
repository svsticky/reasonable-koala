// Composables
import {createRouter, createWebHistory} from 'vue-router'

const routes = [
  {
    path: '/auth',
    component: () => import('@/layouts/login/Login.vue'),
    children: [
      {
        path: 'login',
        name: 'Login',
        component: () => import('@/views/auth/Login.vue')
      },
      {
        path: 'register',
        name: 'Register',
        component: () => import('@/views/auth/Register.vue')
      },
      {
        path: 'authorize',
        name: 'Authorize',
        component: () => import('@/views/auth/Authorize.vue')
      },
      {
        path: 'login-ok',
        name: 'Login OK',
        component: () => import('@/views/auth/LoginOk.vue')
      },
    ]
  },
  {
    path: '/',
    component: () => import('@/layouts/default/Default.vue'),
    children: [
      {
        path: '',
        name: 'Home',
        component: () => import('@/views/Home.vue')
      },
    ],
  },
  {
    path: '/manager',
    component: () => import('@/layouts/default/Default.vue'),
    children: [
      {
        path: 'users',
        name: 'Users',
        component: () => import("@/views/manager/user/Users.vue")
      },
      {
        path: 'cat',
        name: 'CAT Tokens',
        component: () => import('@/views/manager/cat/ConstantAccessTokens.vue')
      },
      {
        path: 'clients',
        name: 'OAuth2 Clients',
        component: () => import('@/views/manager/client/Clients.vue')
      }
    ]
  }
]

const router = createRouter({
  history: createWebHistory('/'),
  routes,
})

export default router
