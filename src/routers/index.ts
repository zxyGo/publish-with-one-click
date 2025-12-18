import { createRouter, createWebHashHistory } from 'vue-router'
import { staticRouter, errorRouter } from './modules/staticRouter'

const router = createRouter({
  history: createWebHashHistory(),
  routes: [...staticRouter, ...errorRouter]
})

router.beforeEach((to, from, next) => {
  // 暂时不需要登录
  next()
  //   if (to.name !== 'Login' && !localStorage.getItem('token')) {
  //     next({ name: 'Login' })
  //   } else {
  //     next()
  //   }
})

router.afterEach(to => {
  document.title = `${to.meta.title || 'Publish with One Click'} - Publish with One Click`
})

export default router
