<template>
  <div>
    <div v-for="item in items" :key="item.id">
      <div v-if="item.type === 'text'">
        <p>{{ item.text }}</p>
      </div>
      <div v-else-if="item.type === 'image'">
        <img :src="item.src" :alt="item.alt" />
      </div>
      <div v-else-if="item.type === 'video'">
        <video :src="item.src" :alt="item.alt" />
      </div>
    </div>
  </div>
</template>
<script>
export default {
  props: {
    items: {
      type: Array,
      required: true
    }
  }
}
</script>
<style lang="scss" scoped>
    img, video {
        max-width: 100%;
    }
</style>
