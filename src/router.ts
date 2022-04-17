import { createRouter, createWebHistory } from 'vue-router'
import Home from './pages/Home.vue'
import Tag from './pages/Tag.vue'
import About from './pages/About.vue'

const routes = [
  {
    path: '/',
    name: 'Home',
    component: Home,
  },
  {
    path: '/tag',
    name: 'Tag',
    component: Tag,
  },
  {
    path: '/about',
    name: 'About',
    component: About,
  },
]

const router = createRouter({
  history: createWebHistory('/'),
  routes,
})

export default router

export const menuRoutes = routes
