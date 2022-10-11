<template>
  <div>
    <n-card
      size="small"
      hoverable
    >

      <!-- 封面 -->
      <template #cover>
        <img class="image-card-cover" v-lazy="convertFileSrc(imagesMeta.cover)" @click="browseImages">
      </template>

      <!-- 标题 -->
      <template #header>
        <span class="image-card-title" @click="browseImages">{{ imagesMeta.title }}</span>
      </template>
      <div>
        <n-text type="info" v-if="imagesMeta.author">
          {{ imagesMeta.author }}
        </n-text>
        <div class="image-card-intro" v-if="imagesMeta.intro">
          <n-blockquote align-text>{{ imagesMeta.intro }}</n-blockquote>
        </div>
      </div>

      <!-- 下方 -->
      <template #action>
        <!-- 标签 -->
        <n-space :size="5">
          <template v-for="(value) in data.tags" :key="value.id">
            <n-tag type="success" size="small" round closable @close="emit('update', Object.assign({}, imagesMeta))">
              {{ value.name }}
            </n-tag>
          </template>
        </n-space>

        <!-- 添加标签+ -->

        <n-space justify="space-around" :size="10">
          <n-button text @click="emit('update', Object.assign({}, imagesMeta))">
            <template #icon>
              <n-icon>
                <Pencil />
              </n-icon>
            </template>
            编辑
          </n-button>
          <n-popconfirm
            @positive-click="emit('remove', imagesMeta.id)"
            positive-text="没错"
            negative-text="算了"
          >
            <template #trigger>
              <n-button text type="error">
                <template #icon>
                  <n-icon>
                    <TrashBin />
                  </n-icon>
                </template>
                移除
              </n-button>
            </template>
            真的要移除吗？
          </n-popconfirm>
        </n-space>
      </template>
    </n-card>
  </div>
</template>

<script>
import { convertFileSrc } from '@tauri-apps/api/tauri'
import { NCard, NBlockquote, NText, NIcon, NButton, NSpace, NPopconfirm, NTag} from 'naive-ui'
import { useRouter } from 'vue-router'
import { Pencil, TrashBin } from '@vicons/ionicons5'
import { reactive } from 'vue'

export default {
  name: 'ImageCard',
  components: {
    NCard,
    NBlockquote,
    NText,
    NIcon,
    NButton,
    NSpace,
    Pencil,
    TrashBin,
    NPopconfirm,
    NTag
  },

  props: {
    imagesMeta: Object
  },
  emits: ['remove', 'update'],
  setup (props, context) {
    const router = useRouter()

    let data = reactive({
      tags: [
        {
          id : 1,
          name: "不该"
        },
        {
          id : 2,
          name: "说好再见"
        }
      ]
      
    })

    async function browseImages () {
      // router.push({
      //   path: '/browse',
      //   query: {
      //     id: props.imagesMeta.id
      //   }
      // })

      router.push({
        path: '/drag',
        query: {

        }
      })
    }

    return {
      emit: context.emit,
      data,
      convertFileSrc,
      browseImages
    }
  }
}
</script>

<style scoped>
.image-card-intro {
  font-size: 0.9em;
}
.image-card-cover {
  max-width: 100%;
}
.image-card-cover:hover, .image-card-title:hover {
  cursor: pointer;
}
</style>
