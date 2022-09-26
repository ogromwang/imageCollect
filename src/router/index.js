import { createWebHistory, createRouter } from "vue-router";
import MyLib from "@/views/MyLibrary";
import Browse from "@/views/Browse";

const routes = [
  {
    path: "/",
    name: "my-lib",
    component: MyLib,
  },
  {
    path: "/browse",
    name: "browse",
    component: Browse,
  },
];

const router = createRouter({
  history: createWebHistory(),
  routes,
});

export default router;