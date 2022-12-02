<template>
    <div>
        <n-menu :options="data.menuOptions"></n-menu>
    </div>
</template>
  
<script>
import { h, reactive } from "vue";
import { NMenu, NIcon } from "naive-ui";
import {
    ImagesOutline as LibIcon,
    PricetagOutline as TagIcon,
    // LogOutOutline as HomeIcon
} from "@vicons/ionicons5";
import { RouterLink } from "vue-router";


export default {
    name: 'MenuRouter',
    components: {
        NMenu
    },

    setup() {

        function renderIcon(icon) {
            return () => h(NIcon, null, { default: () => h(icon) });
        }

        let data = reactive({
            menuOptions: [
                {
                    label: () => h(
                        RouterLink,
                        {
                            to: {
                                path: "/"
                            }
                        },
                        { default: () => "我的图库" }
                    ),
                    key: "mylib",
                    icon: renderIcon(LibIcon)
                },
                {
                    label: () => h(
                        RouterLink,
                        {
                            to: {
                                path: "/"
                            }
                        },
                        { default: () => "标签" }
                    ),
                    key: "mylib",
                    icon: renderIcon(TagIcon)
                },
          
            ]

        })

        return {
            data,
        }

    }

}
</script>