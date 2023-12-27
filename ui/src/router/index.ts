// Composables
import {createRouter, createWebHashHistory, createWebHistory} from 'vue-router'

const routes = [
  {
    path: '/',
    component: () => import('@/layouts/default/Default.vue'),
    children: [
      {
        path: '',
        name: 'Home',
        component: () => import('@/views/Home.vue')
      },
      {
        path: 'login',
        name: 'Login',
        component: () => import('@/views/Login.vue')
      },
      {
        path: 'authorize',
        name: 'Authorize',
        component: () => import('@/views/Authorize.vue')
      },
      {
        path: 'login-ok',
        name: 'Login OK',
        component: () => import('@/views/LoginOk.vue')
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
        component: () => import("@/views/manager/Users.vue")
      }
    ]
  }
]

const router = createRouter({
  history: createWebHistory(),
  routes,
})

export default router
