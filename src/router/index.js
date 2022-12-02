import { createWebHistory, createRouter } from "vue-router";
import MyLib from "@/views/MyLibrary";
import MyFileLib from "@/views/MyFileLib";
import Browse from "@/views/Browse";
import Drag from "@/views/Drag";

const routes = [
  {
    path: "/mylib",
    name: "my-lib",
    component: MyLib,
  },
  {
    path: "/",
    name: "my-file-lib",
    component: MyFileLib,
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