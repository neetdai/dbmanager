import { createMemoryHistory, createRouter, createWebHistory, Router } from "vue-router";
import NewConnection from "./NewConnection.vue";
import HomePage from "./HomePage.vue";

const router: Router = createRouter({
    history: createWebHistory(),
    routes: [
        {
            path: '/',
            component: HomePage,
        },
        {
            path: '/new_connection',
            component: NewConnection,
        }
    ]
});


export default router;
