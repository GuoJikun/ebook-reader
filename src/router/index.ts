import {
  createRouter,
  createWebHistory,
  type RouteRecordRaw,
} from "vue-router";

import Home from "../views/home.vue";
import Detail from "../views/detail.vue";

const routes: Array<RouteRecordRaw> = [
  {
    path: "/",
    name: "home",
    component: Home,
    meta: { title: "书架" },
  },
  {
    path: "/detail",
    name: "detail",
    component: Detail,
    meta: { title: "阅读" },
  },
  {
    path: "settings",
    name: "settings",
    component: () => import("../views/settings.vue"),
    meta: { title: "设置" },
  },
];

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes,
});

export default router;
