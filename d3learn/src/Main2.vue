<template>
  <div id="app">
    <circle-mark :init-points="points[0]"/>

    <!-- <article class="c_container c_small" id="wrap"></article> -->
  </div>
</template>

<script>
import CircleMark from "@/components/CircleMark";

export default {
  name: "App",

  components: {
    CircleMark,
  },

  mounted: function () {
  },

  data: function () {
    return {
      points: [
        [[300, 100], [500, 100]]
      ]
    }
  },
};
</script>

<style>
#app {
  font-family: "Open Sans", Helvetica, Arial, sans-serif;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  text-align: center;
  color: #282f36;
  margin-top: 30px;
}
</style>
