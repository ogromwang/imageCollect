<template>
  <div class="mask" v-show="data.show" id="mask">
    <h3>拖拽到这里上传</h3>
  </div>
</template>

<script>
import {reactive, ref} from "vue";

export default {
  name: "MyDrag",

  setup() {
    let data = reactive({
      tempIndex: 0,
      fileList: [],
      accept: ref<String>('.jpg,.gif,.png,.jpeg'),
      show: true
    });

    // 初始化拖入事件
    function init() {
      console.debug("init 事件")
      const ele = document.querySelector('body')
      console.debug("查询 body,", ele)

      if (ele) {
        // 进入区域
        // ele.addEventListener('dragenter', () => {
        // })

        // 拖离区域
        // ele.addEventListener('dragleave', (e) => {
        //   if (
        //       e.target.nodeName === 'HTML' ||
        //       e.target === e.explicitOriginalTarget ||
        //       (!e.fromElement &&
        //           (e.clientX <= 0 ||
        //               e.clientY <= 0 ||
        //               e.clientX >= window.innerWidth ||
        //               e.clientY >= window.innerHeight))
        //   ) {
        //   }
        // })

        ele.addEventListener('drop', (e) => {
          console.debug("drop")
          onDrop(e)
        }) //拖进

        ele.addEventListener('dragover', (e) => {
          e.preventDefault()
        }) //拖进
      }
    }

    // 拖入时的事件
    const onDrop = (e) => {
      console.debug("拖入事件")
      if (e.dataTransfer) {
        const list = [].slice.call(e.dataTransfer.files).filter((file) => {
          if (data.accept) {
            return checkType(file, data.accept)
          }
          return true
        })
        data.fileList = list.map((p) => {
          return handleStart(p)
        })
        // onChange()

      }
    }
    // 检查文件类型
    const checkType = (file, accept = '') => {
      const {type, name} = file
      if (accept.length === 0) return true
      const extension = name.indexOf('.') > -1 ? `.${name.split('.').pop()}` : ''
      const baseType = type.replace(/\/.*$/, '')
      return accept
          .split(',')
          .map((type) => type.trim())
          .filter((type) => type)
          .some((acceptedType) => {
            if (/\..+$/.test(acceptedType)) {
              return extension === acceptedType
            }
            if (/\/\*$/.test(acceptedType)) {
              return baseType === acceptedType.replace(/\/\*$/, '')
            }
            if (/^[^/]+\/[^/]+$/.test(acceptedType)) {
              return type === acceptedType
            }
          })
    }
    // 处理文件列表返回值
    const handleStart = (rawFile) => {
      rawFile.uid = Date.now() + data.tempIndex++
      return {
        status: 'ready',
        name: rawFile.name,
        size: rawFile.size,
        percentage: 0,
        uid: rawFile.uid,
        raw: rawFile,
      }
    }

    init()

    return {
      data
    }

  },

}
</script>

<style scoped>

.mask {
  height: auto;
  width: auto;
}

</style>