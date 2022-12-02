import { invoke } from '@tauri-apps/api/tauri'

export default {
  async initTable () {
    return invoke('init_table')
  },

  async addImagesMeta (meta) {
    return invoke('add_images_meta', meta)
  },
  
  async getImagesMetaList (search, page, pageSize) {
    const data = {
      search: search,
      page: page,
      pageSize: pageSize
    }
    return invoke('get_images_meta_list', data)
  }, 

  async getFileMetaList (search, page, pageSize) {
    const data = {
      search: search,
      page: page,
      pageSize: pageSize
    }
    return invoke('get_file_meta_list', data)
  },

  async getImagesFolderInfo (path) {
    return invoke('get_images_folder_info', {
      pathStr: path
    })
  },

  async getImagesMeta (id) {
    return invoke('get_images_meta', {
      id
    })
  },

  async updateBrowseSettings (metaId, browseType, homePage, currentPath, currentIndex) {
    return invoke('update_browse_settings', {
      metaId,
      browseType,
      homePage,
      currentPath,
      currentIndex
    })
  },

  async getBrowseSettings (metaId) {
    return invoke('get_browse_settings', {
      metaId
    })
  },

  async deleteImageMeta (id) {
    return invoke('delete_images_meta', {
      id
    })
  },

  async updateImagesMeta (meta) {
    return invoke('update_images_meta', meta)
  },

  // 上传文件
  async uploadFile (bytes, fileName, fileType) {
    return invoke('upload_file', {
      bytes,
      fileName,
      fileType
    })
  },

  // 隐藏窗口
  async hideWindow () {
    return invoke('hide_window')
  }

}