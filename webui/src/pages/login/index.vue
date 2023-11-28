<template>
  <div class="login-wrapper">
    <login-header />

    <div class="login-container">
      <div class="title-container">
        <h1 class="title margin-no">{{ $t('pages.login.loginTitle') }}</h1>
        <h1 class="title">mikanarr</h1>
        <div class="sub-title">
          <p class="tip">{{ $t('pages.login.subtitle') }}</p>
        </div>
      </div>

      <login />
    </div>

    <footer class="copyright">Copyleft © 2023-{{ new Date().getFullYear() }} vuhe. All Rights Reserved</footer>
  </div>
</template>
<script lang="ts">
export default {
  name: 'LoginIndex',
};
</script>
<script setup lang="ts">
import LoginHeader from './components/Header.vue';
import Login from './components/Login.vue';
</script>

<style lang="less" scoped>
@import './index.less';
</style>
