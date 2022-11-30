<template>
  <div class="mask" :class="data.changeColor" v-show="data.show" id="mask">
    <h4>拖拽到这里上传</h4>
  </div>

</template>

<script>
import { reactive, ref } from "vue";

export default {
  name: "MyDrag",

  setup() {
    // class 名称
    const background = {
      on: "on",
      off: "off"
    }

    let data = reactive({
      tempIndex: 0,
      fileList: [],
      accept: ref < String > ('.jpg,.gif,.png,.jpeg'),
      show: true,
      changeColor: background.off
    });

    function changeDragTipColor(inner) {
      if (inner) {
        data.changeColor = background.on
      } else {
        data.changeColor = background.off
      }
    }

    // 初始化拖入事件
    function init() {
      const ele = document.querySelector('body')

      if (ele) {
        // 进入区域
        // ele.addEventListener('dragenter', () => {
        // })

        // 拖离区域
        ele.addEventListener('dragleave', (e) => {
          e.stopPropagation();
          e.preventDefault()
          changeDragTipColor(false)
        })

        ele.addEventListener('drop', (e) => {
          e.stopPropagation();
          e.preventDefault()
          onDrop(e)
        }) //拖进

        ele.addEventListener('dragover', (e) => {
          e.stopPropagation();
          e.preventDefault();
          changeDragTipColor(true)
        }) //拖进
      }
    }

    // 拖入时的事件
    const onDrop = (e) => {
      console.debug("1. 拖入事件开始", e.dataTransfer);

      if (e.dataTransfer) {
        const list = [].slice.call(e.dataTransfer.files).filter((file) => {
          console.log(e.dataTransfer)
          if (data.accept) {
            return checkType(file, data.accept)
          }
          return true
        })

        data.fileList = list.map((p) => {
          return handleStart(p)
        })

        console.debug("2. fileList为 ", data.fileList);

        // onChange()
        changeDragTipColor(false)

        var files = e.dataTransfer.files;

        console.debug("3. files.length ", files.length);

        if (files.length > 0) {
          var reader = new FileReader();
          var fileByteArray = [];
          reader.readAsArrayBuffer(files[0]);
          reader.onloadend = function (evt) {
            if (evt.target.readyState == FileReader.DONE) {
              var arrayBuffer = evt.target.result, array = new Uint8Array(arrayBuffer);
              for (var i = 0; i < array.length; i++) {
                fileByteArray.push(array[i]);
              }
            }

            console.debug("4. 字节数组 ", fileByteArray);
          }
        }
      }
    }

    // 检查文件类型
    const checkType = (file, accept = '') => {
      const { type, name } = file
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

.off {
  background-color: gray;
  font-style: initial;
  color: whitesmoke;
  font-size: small;
  text-align: center;
}

.on {
  background-color: whitesmoke;
  font-style: initial;
  color: black;
  font-size: small;
  text-align: center;
}

div {
    -moz-user-select:none;
    -webkit-user-select:none;
    user-select:none;
}

</style>