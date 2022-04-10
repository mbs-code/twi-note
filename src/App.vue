<!-- <template>
  <n-dialog-provider>
    <n-layout content-style="padding: 8px;">
      <router-view />
    </n-layout>
  </n-dialog-provider>
</template> -->

<template>
  <n-dialog-provider>
    <n-layout position="absolute">
      <n-layout-header position="absolute" bordered style="height: 22px;">
        <div style="background: #f0f8ff">div</div>
      </n-layout-header>

      <n-layout has-sider position="absolute" style="top: 22px;">
        <n-layout content-style="padding: 8px;">
          <router-view />
        </n-layout>
      </n-layout>
    </n-layout>
  </n-dialog-provider>
</template>
