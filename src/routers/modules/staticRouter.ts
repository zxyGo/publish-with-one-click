export const menuRouter = [
  {
    path: '',
    name: 'Publish',
    component: () => import('@/views/publish/index.vue'),
    meta: {
      title: '发布'
    }
  },
  {
    path: 'editor',
    name: 'Editor',
    component: () => import('@/views/md/index.vue'),
    meta: {
      title: 'Markdown 编辑器'
    }
  }
]

export const staticRouter = [
  {
    path: '/',
    name: 'Home',
    component: () => import('@/layout/index.vue'),
    meta: {
      title: 'Home',
      hidden: true
    },
    children: menuRouter
  },
  {
    path: '/login',
    name: 'Login',
    component: () => import('@/views/login/index.vue'),
    meta: {
      title: 'Login',
      hidden: true
    }
  }
]

export const errorRouter = [
  {
    path: '/:pathMatch(.*)*',
    name: 'Error',
    component: () => import('@/views/error/index.vue'),
    meta: {
      title: 'Error',
      hidden: true
    }
  }
]
