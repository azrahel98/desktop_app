<template>
  <div class="nav">
    <div />
    <div class="card border-0">
      <div class="d-flex mx-1">
        <div class="avatar" style="height: 40px; width: 40px">
          <img src="../assets/logo.png" class="img-fluid rounded-circle" />
        </div>
        <button class="btn btn-sm" @click="toggle()">&#9776;</button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
defineProps({
  open: { type: Boolean, required: true },
  toggle: { type: Function, required: true }
})
</script>

<style lang="scss" scoped>
.nav {
  position: absolute;
  right: 0;
  display: flex;
  padding: 0;
  margin: 0;
  justify-content: space-between;
  padding-top: 10px;
  .card {
    margin: 0;
    margin-right: 2vw;
    margin-top: 0vh;
    border-radius: 20px;
    width: 100%;
    display: flex;
    flex-direction: row;
    justify-content: space-between;
    align-items: center;
    padding: 0.5vh 0.3vw 0.6vh 0.6vw;
    height: 40px;

    input {
      border-radius: 20px;
      border: none;
      font-size: 14px;
      font-weight: normal;
    }
    :active,
    :focus-visible,
    :focus {
      box-shadow: none;
    }
    .avatar {
      width: 2.2rem;
      height: 2.2rem;
    }
  }
}
</style>
