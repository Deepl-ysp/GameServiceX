import { createRouter, createWebHistory } from 'vue-router'

const routes = [
  {
    path: '/',
    name: 'home',
    component:() => import("@pages/Home.vue")
  },
  {
    path:'/Select',
    name:'select',
    component:() => import("@pages/SelectLevels.vue")
  },
  {
    path:'/Level',
    name:'level',
    component:() => import("@pages/Level.vue")
  },
  {
    path:'/About',
    name:'about',
    component:() => import("@pages/About.vue")
  },
  {
    path:'/Settings',
    name:'settings',
    component:() => import("@pages/Settings.vue")
  }
]

const router = createRouter({
  history: createWebHistory(),
  routes
})

export default router