import { createWebHistory, createRouter } from "vue-router";
import MyLib from "@/views/MyLibrary";
import Browse from "@/views/Browse";
import Drag from "@/views/Drag";

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
  {
    path: "/drag",
    name: "drag",
    component: Drag,
  },
];

const router = createRouter({
  history: createWebHistory(),
  routes,
});

export default router;